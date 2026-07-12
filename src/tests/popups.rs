//! xdg-popup lifecycle and xdg-activation focus policy driven through a real
//! client: mapping/tracking, parent teardown, grab-serial handling, client
//! crash reaping, and the serial gate on activation.

use super::{
    Fixture, first_popup_surface, keyboard_focus, map_popup, map_window, popups_tracked_on,
    server_surface, window_by_app_id,
};

#[test]
fn popup_maps_and_is_tracked() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));
    let id = f.add_client();

    let parent = map_window(&mut f, id, "parent", (400, 300));
    let popup = map_popup(&mut f, id, &parent);

    let window = window_by_app_id(&mut f, "parent").unwrap();
    let root = server_surface(&window);
    assert_eq!(
        popups_tracked_on(&root),
        1,
        "compositor should track the mapped popup on its parent"
    );

    let cfgs = f.client(id).popup(&popup).format_recent_configures();
    assert!(!cfgs.is_empty(), "popup should have received a configure");
    assert!(
        !f.client(id).popup(&popup).popup_done,
        "a freshly mapped popup must not be dismissed"
    );

    f.client(id).popup(&popup).destroy();
    f.double_roundtrip(id);
}

#[test]
fn popup_orphaned_when_parent_closes_reaps_cleanly() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));
    let id = f.add_client();

    let parent = map_window(&mut f, id, "parent", (400, 300));
    let popup = map_popup(&mut f, id, &parent);

    let window = window_by_app_id(&mut f, "parent").unwrap();
    let root = server_surface(&window);
    let popup_server = first_popup_surface(&root).unwrap();

    // Destroy the parent toplevel out from under the still-open popup.
    f.client(id).window(&parent).destroy();
    f.double_roundtrip(id);
    f.pump(5);
    f.roundtrip(id);

    // driftwm does not proactively dismiss an orphaned popup: no popup_done is
    // sent and it stays tracked. Reaping is deferred to the popup's own
    // teardown (below) or the client's death — never leaked, never a crash.
    assert!(
        !f.client(id).popup(&popup).popup_done,
        "driftwm sends no popup_done on parent close"
    );
    assert!(
        f.state().popups.find_popup(&popup_server).is_some(),
        "orphaned popup stays tracked until its own surface is destroyed"
    );

    // Destroying the popup surface reaps it on the next cleanup pass.
    f.client(id).popup(&popup).destroy();
    f.double_roundtrip(id);
    f.pump(3);
    assert!(
        f.state().popups.find_popup(&popup_server).is_none(),
        "popup must be reaped once its surface is destroyed"
    );
}

#[test]
fn popup_grab_with_unrecognized_serial_is_honored() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));
    let id = f.add_client();

    let parent = map_window(&mut f, id, "parent", (400, 300));

    let popup = f.client(id).create_popup(&parent);
    let popup_surface = popup.surface.clone();
    popup.commit();
    f.roundtrip(id);

    let popup = f.client(id).popup(&popup_surface);
    popup.grab(999_999);
    popup.attach_new_buffer();
    popup.ack_last_and_commit();
    f.double_roundtrip(id);

    // driftwm does not validate the grab serial: the grab is installed and the
    // popup stays mapped rather than being dismissed.
    assert!(
        f.state().popup_grab.is_some(),
        "grab should be installed despite the bogus serial"
    );
    assert!(
        !f.client(id).popup(&popup_surface).popup_done,
        "popup must not be dismissed for an unrecognized grab serial"
    );

    let window = window_by_app_id(&mut f, "parent").unwrap();
    let root = server_surface(&window);
    assert_eq!(popups_tracked_on(&root), 1);

    f.client(id).popup(&popup_surface).destroy();
    f.double_roundtrip(id);
}

#[test]
fn client_crash_with_open_popup_reaps_everything() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));
    let id = f.add_client();

    let parent = map_window(&mut f, id, "parent", (400, 300));

    // A grabbed popup, so the crash also has to reap the popup grab.
    let popup = f.client(id).create_popup(&parent);
    let popup_surface = popup.surface.clone();
    popup.commit();
    f.roundtrip(id);
    let popup = f.client(id).popup(&popup_surface);
    popup.grab(1);
    popup.attach_new_buffer();
    popup.ack_last_and_commit();
    f.double_roundtrip(id);

    let window = window_by_app_id(&mut f, "parent").unwrap();
    let root = server_surface(&window);
    let popup_server = first_popup_surface(&root).unwrap();
    assert!(f.state().popup_grab.is_some());

    f.kill_client(id);
    f.pump(20);

    assert!(
        f.state().popups.find_popup(&popup_server).is_none(),
        "popup must be reaped when its client dies"
    );
    assert!(
        f.state().popup_grab.is_none(),
        "popup grab must be released when its client dies"
    );
}

#[test]
fn activation_with_serial_moves_focus() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));
    let id = f.add_client();

    let target = map_window(&mut f, id, "target", (400, 300));
    let requester = map_window(&mut f, id, "requester", (400, 300));

    let requester_win = window_by_app_id(&mut f, "requester").unwrap();
    assert_eq!(keyboard_focus(&mut f), Some(server_surface(&requester_win)));

    // Token created from user input (carries a serial) → honored.
    f.client(id).request_activation_token(&requester, true);
    f.roundtrip(id);
    f.client(id).activate(&target);
    f.double_roundtrip(id);

    let target_win = window_by_app_id(&mut f, "target").unwrap();
    assert_eq!(
        keyboard_focus(&mut f),
        Some(server_surface(&target_win)),
        "activation with a valid serial must move focus to the target"
    );
}

#[test]
fn activation_without_serial_does_not_move_focus() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));
    let id = f.add_client();

    let target = map_window(&mut f, id, "target", (400, 300));
    let requester = map_window(&mut f, id, "requester", (400, 300));

    let requester_win = window_by_app_id(&mut f, "requester").unwrap();
    let requester_surface = server_surface(&requester_win);
    assert_eq!(keyboard_focus(&mut f), Some(requester_surface.clone()));

    // Token with no serial is a spontaneous attention request → ignored.
    f.client(id).request_activation_token(&requester, false);
    f.roundtrip(id);
    f.client(id).activate(&target);
    f.double_roundtrip(id);

    assert_eq!(
        keyboard_focus(&mut f),
        Some(requester_surface),
        "activation without a serial must not steal focus"
    );
}
