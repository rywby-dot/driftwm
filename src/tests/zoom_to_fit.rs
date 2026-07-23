//! `zoom-to-fit` is a camera toggle, not a mode: it saves the pre-fit camera
//! and zoom so a second press returns there, but any deliberate move — a pan
//! or a navigation — disarms that return and keeps the zoomed-out zoom
//! instead of restoring the saved one.

use std::time::Duration;

use driftwm::config::{Action, Direction};
use smithay::utils::{Logical, Point};

use crate::state::StageWindow;

use super::{Fixture, map_window, window_by_app_id};

const TICK: Duration = Duration::from_millis(16);
const MAX_TICKS: usize = 600;

/// Run both viewport animations to completion, in the order a real frame loop
/// ticks them (zoom first, so the camera uses the recomputed target).
fn settle(f: &mut Fixture) {
    for _ in 0..MAX_TICKS {
        if f.state().camera_target().is_none() && f.state().zoom_target().is_none() {
            return;
        }
        f.state().apply_zoom_animation(TICK);
        f.state().apply_camera_animation(TICK);
    }
    panic!("viewport animation did not converge within {MAX_TICKS} ticks");
}

/// Two windows far enough apart that fitting them needs a zoom well under 1.0.
/// Focus ends on the right-hand one, so `center-nearest` left has a target.
fn two_spread_windows(f: &mut Fixture) {
    f.add_output(1, (1920, 1080));
    // Moving the camera seeds a per-output blur generation that only clears on
    // output disconnect, so it can never return to the pre-output baseline.
    f.skip_baseline_check();
    let id = f.add_client();
    map_window(f, id, "left", (400, 300));
    map_window(f, id, "right", (400, 300));

    let left = window_by_app_id(f, "left").expect("left window");
    let right = window_by_app_id(f, "right").expect("right window");
    f.state()
        .map_window(StageWindow::Client(left), Point::from((0, 0)), false);
    f.state()
        .map_window(StageWindow::Client(right), Point::from((4000, 0)), true);
    settle(f);
}

/// Fit the spread layout and settle there, returning the framing it lands on.
fn enter_fit_view(f: &mut Fixture) -> (f64, Point<f64, Logical>) {
    f.state().execute_action(&Action::ZoomToFit);
    settle(f);
    let fit_zoom = f.state().zoom();
    assert!(
        fit_zoom < 1.0,
        "the layout should need zooming out to fit, got {fit_zoom}"
    );
    (fit_zoom, f.state().camera())
}

#[test]
fn navigating_from_a_fit_view_keeps_the_zoomed_out_zoom() {
    let mut f = Fixture::new();
    two_spread_windows(&mut f);
    let (fit_zoom, _) = enter_fit_view(&mut f);

    f.state()
        .execute_action(&Action::CenterNearest(Direction::Left));
    settle(&mut f);

    // Without this the zoom assert below passes vacuously: a search that finds
    // no target is a no-op, which also leaves the zoom untouched.
    let left = window_by_app_id(&mut f, "left").expect("left window");
    assert!(
        f.state().focused_window() == Some(left),
        "the search reached the left window"
    );
    assert!(
        (f.state().zoom() - fit_zoom).abs() < 1e-6,
        "navigating stays at the fit zoom instead of restoring the pre-fit one, \
         got {} want {fit_zoom}",
        f.state().zoom()
    );
}

#[test]
fn navigating_disarms_the_fit_toggle() {
    let mut f = Fixture::new();
    two_spread_windows(&mut f);
    let (fit_zoom, fit_camera) = enter_fit_view(&mut f);

    f.state()
        .execute_action(&Action::CenterNearest(Direction::Left));
    settle(&mut f);

    // The return is spent, so this press fits afresh — same bbox, so the same
    // framing as the first fit — instead of jumping to the pre-fit viewport.
    f.state().execute_action(&Action::ZoomToFit);
    settle(&mut f);

    assert!(
        (f.state().zoom() - fit_zoom).abs() < 1e-6,
        "a fresh fit lands on the fit zoom, got {}",
        f.state().zoom()
    );
    let camera = f.state().camera();
    assert!(
        (camera.x - fit_camera.x).abs() < 1e-6 && (camera.y - fit_camera.y).abs() < 1e-6,
        "a fresh fit reproduces the first fit's framing, got {camera:?} want {fit_camera:?}"
    );
}

#[test]
fn a_second_press_returns_to_the_pre_fit_viewport() {
    let mut f = Fixture::new();
    two_spread_windows(&mut f);
    let camera_before = f.state().camera();
    let zoom_before = f.state().zoom();

    enter_fit_view(&mut f);
    f.state().execute_action(&Action::ZoomToFit);
    settle(&mut f);

    assert!(
        (f.state().zoom() - zoom_before).abs() < 1e-6,
        "the toggle restores the pre-fit zoom, got {} want {zoom_before}",
        f.state().zoom()
    );
    let camera = f.state().camera();
    assert!(
        (camera.x - camera_before.x).abs() < 1e-6 && (camera.y - camera_before.y).abs() < 1e-6,
        "the toggle restores the pre-fit camera, got {camera:?} want {camera_before:?}"
    );
}
