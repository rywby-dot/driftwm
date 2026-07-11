//! Exact configure sequences as the client sees them — the desync class where
//! a toolkit acks one configure while the compositor already believes another.

use super::{Fixture, window_by_app_id};

/// Map one toplevel with a buffer at `size`, settle, and drain the configure
/// cursor so tests only see what happens next.
fn map_settled(
    f: &mut Fixture,
    id: super::client::ClientId,
    app_id: &str,
    size: (u16, u16),
) -> wayland_client::protocol::wl_surface::WlSurface {
    let window = f.client(id).create_window();
    let surface = window.surface.clone();
    window.set_app_id(app_id);
    window.commit();
    f.roundtrip(id);

    let window = f.client(id).window(&surface);
    window.set_size(size.0, size.1);
    window.attach_new_buffer();
    window.ack_last_and_commit();
    f.double_roundtrip(id);
    f.client(id).window(&surface).format_recent_configures();
    surface
}

#[test]
fn initial_burst_is_a_single_configure() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));
    let id = f.add_client();

    let window = f.client(id).create_window();
    let surface = window.surface.clone();
    window.commit();
    f.double_roundtrip(id);

    // Exactly one configure before the first ack — a second uncommitted
    // configure in the initial burst is what desyncs size-tracking toolkits.
    let window = f.client(id).window(&surface);
    assert_eq!(window.recent_configures().count(), 1);
}

#[test]
fn fullscreen_reassert_is_idempotent() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));
    let id = f.add_client();

    let surface = map_settled(&mut f, id, "fs", (800, 600));
    let window = f.client(id).window(&surface);
    window.set_fullscreen(None);
    f.double_roundtrip(id);
    f.client(id).window(&surface).ack_last_and_commit();
    f.double_roundtrip(id);
    f.client(id).window(&surface).format_recent_configures();

    // Toolkits re-assert fullscreen on focus changes; the answer must be the
    // same fullscreen configure again, never an exit/re-enter bounce.
    let window = f.client(id).window(&surface);
    window.set_fullscreen(None);
    f.double_roundtrip(id);

    let window = f.client(id).window(&surface);
    let configures = window.format_recent_configures();
    for line in configures.lines() {
        assert!(
            line.contains("size: 1920 × 1080") && line.contains("Fullscreen"),
            "re-assert must only repeat the fullscreen configure, got:\n{configures}"
        );
    }
}

#[test]
fn second_fullscreen_displaces_first() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));
    let id = f.add_client();

    let first = map_settled(&mut f, id, "first", (800, 600));
    let second = map_settled(&mut f, id, "second", (400, 300));

    let window = f.client(id).window(&first);
    window.set_fullscreen(None);
    f.double_roundtrip(id);
    f.client(id).window(&first).ack_last_and_commit();
    f.double_roundtrip(id);
    f.client(id).window(&first).format_recent_configures();

    let window = f.client(id).window(&second);
    window.set_fullscreen(None);
    f.double_roundtrip(id);

    // The displaced window is restored to its pre-fullscreen size...
    let first_configures = f.client(id).window(&first).format_recent_configures();
    assert!(
        first_configures.contains("size: 800 × 600") && !first_configures.contains("Fullscreen"),
        "displaced window must get its windowed configure back, got:\n{first_configures}"
    );
    // ...and the new one owns the output.
    let second_configures = f.client(id).window(&second).format_recent_configures();
    assert!(
        second_configures.contains("size: 1920 × 1080") && second_configures.contains("Fullscreen"),
        "takeover window must get the fullscreen configure, got:\n{second_configures}"
    );
    let mapped = window_by_app_id(&mut f, "second").unwrap();
    assert_eq!(
        f.state().stage.fullscreen_output_of(&mapped),
        Some("HEADLESS-1")
    );
}

#[test]
fn fit_round_trip_restores_exact_size() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));
    let id = f.add_client();

    // Even size on purpose: odd fitted sizes hit a known pre-existing 1px
    // truncation quirk that is not under test here.
    let surface = map_settled(&mut f, id, "fit", (800, 600));
    let window = window_by_app_id(&mut f, "fit").unwrap();

    f.state().toggle_fit_window(&window);
    f.double_roundtrip(id);
    let client_window = f.client(id).window(&surface);
    let fit_configures = client_window.format_recent_configures();
    assert!(
        fit_configures.contains("size:") && !fit_configures.contains("size: 800 × 600"),
        "fit must configure a new (viewport-fitted) size, got:\n{fit_configures}"
    );
    // Commit at the fitted size so the exit path restores from a fit-sized
    // window, as a real client would.
    let (w, h) = client_window.configures_received.last().unwrap().1.size;
    let client_window = f.client(id).window(&surface);
    client_window.set_size(w as u16, h as u16);
    client_window.ack_last_and_commit();
    f.double_roundtrip(id);

    f.state().toggle_fit_window(&window);
    f.double_roundtrip(id);
    let configures = f.client(id).window(&surface).format_recent_configures();
    assert!(
        configures.contains("size: 800 × 600"),
        "fit exit must restore the exact pre-fit size, got:\n{configures}"
    );
    assert!(!f.state().stage.is_fit(&window));
}
