//! Alt-Tab cycle sessions commit the selected window on the first non-cycle
//! focus change or action. Modifier-less cycle sources (gestures, bare wheel)
//! have no keyboard release to end the session, so `focus_changed` and
//! `execute_action` both watch for it; a cycle step's own navigate is exempt
//! via the `cycle_navigating` flag.

use driftwm::config::Action;
use smithay::utils::SERIAL_COUNTER;

use super::{Fixture, client::ClientId, map_window, window_by_app_id};

/// Map toplevels a, b, c on a fresh client; focus history ends [c, b, a].
/// Returns the client so callers can map more windows.
fn three_windows(f: &mut Fixture) -> ClientId {
    let id = f.add_client();
    map_window(f, id, "a", (400, 300));
    map_window(f, id, "b", (400, 300));
    map_window(f, id, "c", (400, 300));
    id
}

fn cycle_forward(f: &mut Fixture) {
    f.state()
        .execute_action(&Action::CycleWindows { backward: false });
}

#[test]
fn cycle_steps_dig_deeper_without_committing() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));
    three_windows(&mut f);

    let before = f.state().stage.focus_history().to_vec();
    cycle_forward(&mut f);
    let after_one = f.state().stage.cycle_state();
    cycle_forward(&mut f);
    let after_two = f.state().stage.cycle_state();

    // The exemption flag holds: a cycle step's own focus change neither commits
    // nor promotes, so the session stays open and the history stays frozen.
    assert!(after_one.is_some(), "first step opens a cycle");
    assert!(after_two.is_some(), "second step keeps cycling");
    assert_ne!(after_one, after_two, "the second step digs deeper");
    assert_eq!(
        f.state().stage.focus_history().to_vec(),
        before,
        "cycling must not reorder focus history until the session ends"
    );
}

#[test]
fn non_cycle_action_commits_the_selection() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));
    three_windows(&mut f);

    cycle_forward(&mut f);
    cycle_forward(&mut f);
    let idx = f.state().stage.cycle_state().expect("mid-cycle");
    let selected = f.state().stage.focus_history()[idx].clone();

    // A non-focus-changing action ends the session via the execute_action choke.
    f.state().execute_action(&Action::ZoomReset);

    assert_eq!(f.state().stage.cycle_state(), None, "the session ended");
    assert_eq!(
        f.state().stage.focus_history().first(),
        Some(&selected),
        "the selected window is committed to the MRU head"
    );
}

#[test]
fn click_focus_during_cycle_commits_then_promotes() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));
    three_windows(&mut f);

    cycle_forward(&mut f);
    cycle_forward(&mut f);
    let idx = f.state().stage.cycle_state().expect("mid-cycle");
    let selected = f.state().stage.focus_history()[idx].clone();

    // A real focus change through the click-to-focus path: raise_and_focus runs
    // update_keyboard_focus → focus_changed, hitting the cycle choke. Focus a
    // window other than the selection.
    let clicked = window_by_app_id(&mut f, "c").unwrap();
    assert_ne!(
        clicked, selected,
        "focus a different window than the selection"
    );
    let serial = SERIAL_COUNTER.next_serial();
    f.state().raise_and_focus(&clicked, serial);

    assert_eq!(f.state().stage.cycle_state(), None, "the session ended");
    let history = f.state().stage.focus_history();
    assert_eq!(history.first(), Some(&clicked), "clicked window on top");
    assert_eq!(
        history.get(1),
        Some(&selected),
        "the selection committed just under the clicked window, not lost"
    );
}

#[test]
fn mapping_a_window_during_cycle_commits_the_selection() {
    let mut f = Fixture::new();
    f.add_output(1, (1920, 1080));
    let id = three_windows(&mut f);

    cycle_forward(&mut f);
    cycle_forward(&mut f);
    let idx = f.state().stage.cycle_state().expect("mid-cycle");
    let selected = f.state().stage.focus_history()[idx].clone();

    // A newly mapped window takes focus at map time → focus_changed → the cycle
    // choke commits the selection before the new window promotes.
    map_window(&mut f, id, "d", (400, 300));
    let mapped = window_by_app_id(&mut f, "d").unwrap();

    assert_eq!(f.state().stage.cycle_state(), None, "the session ended");
    let history = f.state().stage.focus_history();
    assert_eq!(history.first(), Some(&mapped), "the new window is on top");
    assert_eq!(
        history.get(1),
        Some(&selected),
        "the selection committed just under the new window, not lost"
    );
}
