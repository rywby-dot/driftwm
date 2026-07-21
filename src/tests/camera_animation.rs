//! Camera and zoom animations. `pan-viewport` extends `camera_target` and lets
//! `apply_camera_animation` lerp the camera there, warping the pointer by each
//! camera delta so the cursor keeps its screen position.
//! Combined zoom+camera animations pin the anchor's canvas point at a fixed
//! screen point while zoom lerps to target, and finish both coordinates in the
//! same tick — zoom snaps to target but keeps animating while the anchor is
//! still off its screen point, and there is never a camera-only handoff tail.

use std::time::Duration;

use smithay::utils::{Logical, Point};

use driftwm::config::{Action, Direction};

use crate::state::ZoomAnimationAnchor;

use super::Fixture;

const TICK: Duration = Duration::from_millis(16);
const MAX_TICKS: usize = 600;

fn approx(a: Point<f64, Logical>, b: Point<f64, Logical>, tol: f64) -> bool {
    (a.x - b.x).abs() <= tol && (a.y - b.y).abs() <= tol
}

fn dist_sq(a: Point<f64, Logical>, b: Point<f64, Logical>) -> f64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    dx * dx + dy * dy
}

/// Canvas point currently shown at screen point `s`: `camera + s / zoom`.
fn point_at_screen(f: &mut Fixture, s: Point<f64, Logical>) -> Point<f64, Logical> {
    let camera = f.state().camera();
    let zoom = f.state().zoom();
    Point::from((camera.x + s.x / zoom, camera.y + s.y / zoom))
}

fn run_camera_animation(f: &mut Fixture) {
    for _ in 0..MAX_TICKS {
        if f.state().camera_target().is_none() {
            return;
        }
        f.state().apply_camera_animation(TICK);
    }
    panic!("camera animation did not converge within {MAX_TICKS} ticks");
}

fn run_zoom_animation(f: &mut Fixture) {
    for _ in 0..MAX_TICKS {
        if f.state().zoom_target().is_none() {
            return;
        }
        f.state().apply_zoom_animation(TICK);
    }
    panic!("zoom animation did not converge within {MAX_TICKS} ticks");
}

/// A pan action leaves the camera put and sets a target one step away; a second
/// pan extends the target from the target, not from the unmoved camera.
#[test]
fn pan_viewport_sets_target_instead_of_jumping() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));

    let camera = f.state().camera();
    let step = f.state().config.pan_step / f.state().zoom();
    let (ux, uy) = Direction::Right.to_unit_vec();
    let delta = Point::from((ux * step, uy * step));

    f.state()
        .execute_action(&Action::PanViewport(Direction::Right));

    assert!(
        approx(f.state().camera(), camera, 1e-9),
        "a pan must not move the camera directly"
    );
    assert!(
        approx(f.state().camera_target().unwrap(), camera + delta, 1e-9),
        "a pan sets the target one step from the camera"
    );

    f.state()
        .execute_action(&Action::PanViewport(Direction::Right));

    assert!(approx(f.state().camera(), camera, 1e-9));
    assert!(
        approx(
            f.state().camera_target().unwrap(),
            camera + delta + delta,
            1e-9
        ),
        "a repeated pan extends the target from the target, not the camera"
    );
}

/// The camera lerps onto the target and clears it on arrival.
#[test]
fn pan_viewport_converges_and_clears_target() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));
    f.skip_baseline_check();

    f.state()
        .execute_action(&Action::PanViewport(Direction::Right));
    let target = f
        .state()
        .camera_target()
        .expect("a pan sets a camera target");

    run_camera_animation(&mut f);

    assert!(
        f.state().camera_target().is_none(),
        "the target clears when the camera arrives"
    );
    assert!(
        approx(f.state().camera(), target, 1e-6),
        "the camera settles exactly on the target"
    );
}

/// Every camera tick warps the pointer by the camera delta, so the cursor's
/// screen position is unchanged across the whole pan.
#[test]
fn pan_keeps_pointer_screen_position() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));
    f.skip_baseline_check();

    let camera_before = f.state().camera();
    let pointer_before = f.state().seat.get_pointer().unwrap().current_location();

    f.state()
        .execute_action(&Action::PanViewport(Direction::Right));
    for _ in 0..MAX_TICKS {
        if f.state().camera_target().is_none() {
            break;
        }
        f.state().apply_camera_animation(TICK);
        let camera_delta = f.state().camera() - camera_before;
        let pointer_delta =
            f.state().seat.get_pointer().unwrap().current_location() - pointer_before;
        assert!(
            approx(pointer_delta, camera_delta, 1e-6),
            "the pointer shifts by the camera delta on every tick, not just overall"
        );
    }
    assert!(
        f.state().camera_target().is_none(),
        "camera animation did not converge within {MAX_TICKS} ticks"
    );
}

/// A zoom animation with the anchor's canvas point already at its screen point
/// keeps that point pinned every tick while zoom lerps to target, then clears
/// cleanly with no camera-only tail.
#[test]
fn zoom_anchor_holds_screen_point() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));
    f.skip_baseline_check();

    let s = Point::from((960.0, 540.0));
    let camera = Point::from((100.0, 50.0));
    // The canvas point shown at S right now, so only zoom animates.
    let c = Point::from((camera.x + s.x, camera.y + s.y));
    f.state().with_output_state(|os| {
        os.camera = camera;
        os.zoom = 1.0;
        os.zoom_target = Some(0.5);
        os.zoom_animation_anchor = Some(ZoomAnimationAnchor {
            canvas: c,
            screen: s,
        });
        os.camera_target = None;
        os.overview_return = None;
    });

    let mut prev = dist_sq(point_at_screen(&mut f, s), c);
    let mut converged = false;
    for _ in 0..MAX_TICKS {
        f.state().apply_zoom_animation(TICK);
        let d = dist_sq(point_at_screen(&mut f, s), c);
        assert!(
            d <= prev + 1e-6,
            "the screen anchor drifted off its canvas point"
        );
        prev = d;
        if f.state().zoom_target().is_none() {
            converged = true;
            break;
        }
    }
    assert!(
        converged,
        "zoom animation did not converge within {MAX_TICKS} ticks"
    );

    assert_eq!(f.state().zoom(), 0.5, "zoom lands exactly on target");
    assert!(
        approx(point_at_screen(&mut f, s), c, 1e-9),
        "the anchor's canvas point ends at its screen point"
    );
    assert!(f.state().zoom_animation_anchor().is_none());
    assert!(
        f.state().camera_target().is_none(),
        "there is no camera-only handoff tail"
    );
}

/// The coupled-finish invariant: when zoom reaches its close band it snaps to
/// target, but the animation stays alive while the anchor is still off its
/// screen point — and it drives the camera directly, never handing off through
/// `camera_target`. Both coordinates then clear in the same tick.
#[test]
fn zoom_finish_is_coupled() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));
    f.skip_baseline_check();

    let s = Point::from((960.0, 540.0));
    let camera = Point::from((100.0, 50.0));
    let zoom = 0.4995;
    // Displace the anchor's canvas point ~100px from the point now shown at S.
    let at_screen: Point<f64, Logical> =
        Point::from((camera.x + s.x / zoom, camera.y + s.y / zoom));
    let c = Point::from((at_screen.x + 100.0, at_screen.y));
    f.state().with_output_state(|os| {
        os.camera = camera;
        os.zoom = zoom;
        os.zoom_target = Some(0.5);
        os.zoom_animation_anchor = Some(ZoomAnimationAnchor {
            canvas: c,
            screen: s,
        });
        os.camera_target = None;
        os.overview_return = None;
    });

    f.state().apply_zoom_animation(TICK);

    assert_eq!(
        f.state().zoom(),
        0.5,
        "zoom snaps to target inside the close band"
    );
    assert!(
        f.state().zoom_target().is_some(),
        "the animation keeps running while the anchor converges"
    );
    assert!(
        f.state().camera_target().is_none(),
        "the anchor drives the camera directly, no handoff"
    );

    run_zoom_animation(&mut f);

    assert!(f.state().zoom_animation_anchor().is_none());
    assert!(f.state().camera_target().is_none());
    let expected_camera = Point::from((c.x - s.x / 0.5, c.y - s.y / 0.5));
    assert!(
        approx(f.state().camera(), expected_camera, 1e-9),
        "the camera lands exactly where the finish places it, not one lerp short"
    );
}

/// A keyboard zoom action anchors on the viewport center: the anchor's screen
/// point is the usable center and its canvas point is what that center shows,
/// which ends back under the center at the new zoom.
#[test]
fn zoom_action_anchors_at_viewport_center() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));
    f.skip_baseline_check();

    let camera = f.state().camera();
    let zoom = f.state().zoom();
    let center = f.state().usable_center_screen();

    f.state().execute_action(&Action::ZoomOut);

    let anchor = f
        .state()
        .zoom_animation_anchor()
        .expect("a zoom action arms the anchor");
    assert!(
        approx(anchor.screen, center, 1e-9),
        "the anchor screen point is the viewport center"
    );
    let expected_canvas = Point::from((camera.x + center.x / zoom, camera.y + center.y / zoom));
    assert!(
        approx(anchor.canvas, expected_canvas, 1e-9),
        "the anchor canvas point is what the viewport center shows"
    );

    run_zoom_animation(&mut f);

    assert!(
        approx(point_at_screen(&mut f, center), anchor.canvas, 1e-9),
        "the anchor's canvas point ends back under the viewport center"
    );
}
