//! Runtime per-window opacity over the IPC `opacity` verb. The stored
//! `AppliedWindowRule` is the single source of truth: a bare window reads
//! `1.0`, the setter overwrites it in place, and a rule-seeded window reads
//! its rule value.

use super::{Fixture, config, map_window, window_by_app_id};
use crate::ipc::dispatch;
use crate::ipc::protocol::{Request, Response, WindowSelector};
use driftwm::window_ext::WindowExt;

fn read_opacity(f: &mut Fixture, window: Option<WindowSelector>) -> f64 {
    match dispatch(
        Request::Opacity {
            window,
            value: None,
        },
        f.state(),
    ) {
        Ok(Response::Opacity(v)) => v,
        other => panic!("expected an Opacity reply, got {other:?}"),
    }
}

#[test]
fn bare_window_reads_full_opacity() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));
    let id = f.add_client();
    map_window(&mut f, id, "term", (400, 300));

    assert_eq!(read_opacity(&mut f, None), 1.0);
}

#[test]
fn set_then_get_roundtrips() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));
    let id = f.add_client();
    map_window(&mut f, id, "term", (400, 300));

    let set = dispatch(
        Request::Opacity {
            window: None,
            value: Some(0.4),
        },
        f.state(),
    );
    assert_eq!(set, Ok(Response::Opacity(0.4)));
    assert_eq!(read_opacity(&mut f, None), 0.4);
}

#[test]
fn out_of_range_value_errors() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));
    let id = f.add_client();
    map_window(&mut f, id, "term", (400, 300));

    for bad in [-0.1, 1.5, f64::NAN, f64::INFINITY] {
        assert!(
            dispatch(
                Request::Opacity {
                    window: None,
                    value: Some(bad),
                },
                f.state(),
            )
            .is_err(),
            "opacity {bad} must be rejected"
        );
    }
    // A rejected set leaves the stored value untouched.
    assert_eq!(read_opacity(&mut f, None), 1.0);
}

#[test]
fn id_selector_targets_unfocused_window() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));
    let id = f.add_client();
    map_window(&mut f, id, "first", (400, 300));
    map_window(&mut f, id, "second", (400, 300));

    // `second` mapped last, so it holds focus; target `first` by id.
    let first = window_by_app_id(&mut f, "first").unwrap();
    let first_id = f.state().stage.id_of(&first).unwrap().0;

    let set = dispatch(
        Request::Opacity {
            window: Some(WindowSelector::Id(first_id)),
            value: Some(0.6),
        },
        f.state(),
    );
    assert_eq!(set, Ok(Response::Opacity(0.6)));

    assert_eq!(
        read_opacity(&mut f, Some(WindowSelector::Id(first_id))),
        0.6
    );
    // The focused window (`second`) is untouched.
    assert_eq!(read_opacity(&mut f, None), 1.0);
}

#[test]
fn rule_seeded_window_reads_rule_value() {
    let mut f = Fixture::with_config(config(
        r#"
[[window_rules]]
app_id = "dim"
opacity = 0.3
"#,
    ));
    f.add_output(1, (1920, 1080));
    let id = f.add_client();
    map_window(&mut f, id, "dim", (400, 300));

    assert_eq!(read_opacity(&mut f, None), 0.3);
}

#[test]
fn set_preserves_other_rule_derived_fields() {
    let mut f = Fixture::with_config(config(
        r#"
[[window_rules]]
app_id = "dim"
opacity = 0.3
widget = true
"#,
    ));
    f.add_output(1, (1920, 1080));
    let id = f.add_client();
    map_window(&mut f, id, "dim", (400, 300));

    // Widgets never take focus, so reach it by id rather than the default
    // (focused) selector.
    let window = window_by_app_id(&mut f, "dim").unwrap();
    let window_id = f.state().stage.id_of(&window).unwrap().0;

    let set = dispatch(
        Request::Opacity {
            window: Some(WindowSelector::Id(window_id)),
            value: Some(0.7),
        },
        f.state(),
    );
    assert_eq!(set, Ok(Response::Opacity(0.7)));

    assert_eq!(
        read_opacity(&mut f, Some(WindowSelector::Id(window_id))),
        0.7
    );
    // The rule's other field must survive the opacity-only field update.
    assert!(window.is_widget());
}

#[test]
fn app_id_selector_matches_case_insensitive_substring() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));
    let id = f.add_client();
    map_window(&mut f, id, "alpha", (400, 300));
    map_window(&mut f, id, "beta", (400, 300));

    // `beta` mapped last, so it holds focus; target `alpha` by an
    // uppercase substring of its lowercase app_id.
    let set = dispatch(
        Request::Opacity {
            window: Some(WindowSelector::AppId("ALPH".into())),
            value: Some(0.5),
        },
        f.state(),
    );
    assert_eq!(set, Ok(Response::Opacity(0.5)));

    assert_eq!(
        read_opacity(&mut f, Some(WindowSelector::AppId("ALPH".into()))),
        0.5
    );
    // The focused window (`beta`) is untouched.
    assert_eq!(read_opacity(&mut f, None), 1.0);
}
