//! Bookmarks: the flat name → canvas-point registry seeded from config and
//! mutated by the set-bookmark / move-to-bookmark actions, the IPC `bookmark`
//! verb, session restore, and the hot-reload diff. `go-to-bookmark` reuses the
//! `go-to` camera math, so set → go-to round-trips exactly.

use std::collections::BTreeMap;

use driftwm::config::Action;
use driftwm::session::{self, SessionEnvelope};

use super::real::TempDir;
use super::{Fixture, config, map_window, window_by_app_id};
use crate::ipc::dispatch;
use crate::ipc::protocol::{Request, Response};

fn bookmark_request(name: Option<&str>, to: Option<(f64, f64)>, delete: bool) -> Request {
    Request::Bookmark {
        name: name.map(str::to_string),
        to,
        delete,
    }
}

#[test]
fn set_bookmark_then_go_to_round_trips_the_camera() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));

    // GoToPosition stashes a camera target (the destination of the animation).
    f.state()
        .execute_action(&Action::GoToPosition(500.0, -300.0));
    let dest = f
        .state()
        .camera_target()
        .expect("go-to sets a camera target");

    // set-bookmark captures the pending destination, not a mid-flight frame,
    // and stores the exact canvas point that go-to would center on.
    f.state().execute_action(&Action::SetBookmark("a".into()));
    assert_eq!(f.state().bookmarks["a"], [500.0, -300.0]);

    // Move the target elsewhere, then jump back through the bookmark.
    f.state()
        .execute_action(&Action::GoToPosition(-2000.0, 1000.0));
    f.state().execute_action(&Action::GoToBookmark("a".into()));

    let restored = f
        .state()
        .camera_target()
        .expect("go-to-bookmark sets a target");
    assert!((restored.x - dest.x).abs() < 1e-6);
    assert!((restored.y - dest.y).abs() < 1e-6);
    // Bookmarks never touch zoom.
    assert!(f.state().zoom_target().is_none());
}

#[test]
fn go_to_bookmark_unset_name_is_a_no_op() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));

    f.state()
        .execute_action(&Action::GoToPosition(100.0, 100.0));
    let before = f.state().camera_target();
    f.state()
        .execute_action(&Action::GoToBookmark("nope".into()));
    assert_eq!(f.state().camera_target(), before);
}

#[test]
fn move_to_bookmark_places_the_focused_window_center() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));
    let id = f.add_client();
    map_window(&mut f, id, "term", (400, 300));
    let window = window_by_app_id(&mut f, "term").unwrap();

    f.state().bookmarks.insert("b".into(), [300.0, -200.0]);
    f.state()
        .execute_action(&Action::MoveToBookmark("b".into()));

    let loc = f.state().stage.position_of(&window).unwrap();
    let (x, y) = driftwm::canvas::internal_to_rule(loc, window.geometry().size);
    assert_eq!((x, y), (300, -200));
}

#[test]
fn move_to_bookmark_refuses_a_pinned_window() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));
    let id = f.add_client();
    map_window(&mut f, id, "term", (400, 300));
    let window = window_by_app_id(&mut f, "term").unwrap();

    f.state().execute_action(&Action::TogglePinToScreen);
    assert!(f.state().is_pinned(&window));

    f.state().bookmarks.insert("b".into(), [300.0, -200.0]);
    f.state()
        .execute_action(&Action::MoveToBookmark("b".into()));
    // The pinned window is left in screen space, not re-mapped onto the canvas.
    assert!(f.state().is_pinned(&window));
}

#[test]
fn ipc_list_get_set_delete_round_trip_and_errors() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));

    // set / create.
    assert_eq!(
        dispatch(
            bookmark_request(Some("home"), Some((10.0, 20.0)), false),
            f.state()
        ),
        Ok(Response::Bookmark { x: 10.0, y: 20.0 })
    );
    // get.
    assert_eq!(
        dispatch(bookmark_request(Some("home"), None, false), f.state()),
        Ok(Response::Bookmark { x: 10.0, y: 20.0 })
    );
    // list — the four seeded corners plus the new one, sorted.
    match dispatch(bookmark_request(None, None, false), f.state()) {
        Ok(Response::Bookmarks(map)) => {
            assert_eq!(map["home"], [10.0, 20.0]);
            assert!(map.contains_key("1"));
            assert_eq!(map.len(), 5);
        }
        other => panic!("expected a Bookmarks reply, got {other:?}"),
    }
    // get unknown → error.
    assert_eq!(
        dispatch(bookmark_request(Some("nope"), None, false), f.state()),
        Err("no bookmark named 'nope'".to_string())
    );
    // non-finite set → error.
    assert_eq!(
        dispatch(
            bookmark_request(Some("bad"), Some((f64::NAN, 0.0)), false),
            f.state()
        ),
        Err("bookmark coordinates must be finite".to_string())
    );
    // delete.
    assert_eq!(
        dispatch(bookmark_request(Some("home"), None, true), f.state()),
        Ok(Response::Ok)
    );
    // delete again (now unknown) → same wording as get.
    assert_eq!(
        dispatch(bookmark_request(Some("home"), None, true), f.state()),
        Err("no bookmark named 'home'".to_string())
    );
    // delete without a name → error.
    assert_eq!(
        dispatch(bookmark_request(None, None, true), f.state()),
        Err("bookmark delete requires a name".to_string())
    );
    // delete combined with coordinates → error.
    assert_eq!(
        dispatch(
            bookmark_request(Some("home"), Some((0.0, 0.0)), true),
            f.state()
        ),
        Err("bookmark delete does not take coordinates".to_string())
    );
}

/// Write an envelope carrying `bookmarks` to `path`.
fn write_envelope(path: &std::path::Path, bookmarks: BTreeMap<String, [f64; 2]>) {
    let envelope = SessionEnvelope {
        version: session::VERSION,
        saved_at: 0,
        entries: Vec::new(),
        outputs: BTreeMap::new(),
        bookmarks,
    };
    session::write(path, &envelope, false).unwrap();
}

#[test]
fn session_always_serializes_the_registry() {
    let tmp = TempDir::new();
    let path = tmp.path().join("session.json");

    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));
    f.state().session_store.path = Some(path.clone());
    f.state().bookmarks.insert("custom".into(), [42.0, -7.0]);
    f.state().session_store_write_now();

    let envelope = session::read(&path);
    assert_eq!(envelope.bookmarks["custom"], [42.0, -7.0]);
    // Seeds ride along too.
    assert_eq!(envelope.bookmarks["1"], [-1750.0, 1750.0]);
}

#[test]
fn session_restore_off_keeps_config_seeds() {
    let tmp = TempDir::new();
    let path = tmp.path().join("session.json");
    write_envelope(
        &path,
        BTreeMap::from([
            ("custom".to_string(), [1.0, 2.0]),
            ("1".to_string(), [9.0, 9.0]),
        ]),
    );

    let mut f = Fixture::with_config(config("[session]\nrestore_bookmarks = false\n"));
    f.add_output(1, (1920, 1080));
    f.state().session_store.path = Some(path);
    f.state().load_session();

    // The saved registry is not overlaid: seeds stand, saved-only names absent.
    assert!(!f.state().bookmarks.contains_key("custom"));
    assert_eq!(f.state().bookmarks["1"], [-1750.0, 1750.0]);
}

#[test]
fn session_restore_on_overlays_and_seeds_fill_gaps() {
    let tmp = TempDir::new();
    let path = tmp.path().join("session.json");
    write_envelope(
        &path,
        BTreeMap::from([
            ("custom".to_string(), [1.0, 2.0]),
            ("1".to_string(), [9.0, 9.0]),
        ]),
    );

    let mut f = Fixture::with_config(config("[session]\nrestore_bookmarks = true\n"));
    f.add_output(1, (1920, 1080));
    f.state().session_store.path = Some(path);
    f.state().load_session();

    // Restored names win, including over a config seed of the same name…
    assert_eq!(f.state().bookmarks["custom"], [1.0, 2.0]);
    assert_eq!(f.state().bookmarks["1"], [9.0, 9.0]);
    // …and seeds fill the names the save lacks.
    assert_eq!(f.state().bookmarks["2"], [1750.0, 1750.0]);
}

#[test]
fn reload_preserves_runtime_bookmark_reasserts_changed_and_drops_removed() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));

    // A runtime-only bookmark and a runtime override of a seeded one.
    f.state().bookmarks.insert("runtime".into(), [5.0, 6.0]);
    f.state().bookmarks.insert("1".into(), [999.0, 999.0]);

    // Reload with a full four-corner table where only "1" differs from the old
    // (default) config, and "4" is dropped from the config entirely.
    f.state().reload_config_from_contents(
        "[navigation.bookmarks]\n\
         \"1\" = [10, 20]\n\
         \"2\" = [1750, 1750]\n\
         \"3\" = [1750, -1750]\n",
    );

    // "runtime" was in neither config table → the runtime value survives.
    assert_eq!(f.state().bookmarks["runtime"], [5.0, 6.0]);
    // "1" changed in config → re-asserts over the runtime override.
    assert_eq!(f.state().bookmarks["1"], [10.0, 20.0]);
    // "4" was dropped from config → removed from the registry.
    assert!(!f.state().bookmarks.contains_key("4"));
    // Unchanged seeds stay.
    assert_eq!(f.state().bookmarks["2"], [1750.0, 1750.0]);

    // Reload queues a headless-output mode intent the fixture never drains.
    f.state().pending_mode_changes.clear();
}
