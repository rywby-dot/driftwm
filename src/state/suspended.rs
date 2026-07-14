//! Interaction and lifecycle for suspended windows (the compositor-drawn
//! stand-ins left behind when a window is suspended). Rendering lives in the
//! render module; this is focus, relaunch, and dismissal.
//!
//! Relaunch is a stub here — chunk 5 (relaunch + matching) fills
//! [`DriftWm::relaunch_suspended`] and the pending-launch state that
//! [`DriftWm::is_suspended_launching`] reads.

use std::rc::Rc;

use smithay::utils::{IsAlive, SERIAL_COUNTER};

use crate::decorations::DecorationKey;
use crate::state::{DriftWm, StageWindow, SuspendedId, SuspendedWindow};

impl DriftWm {
    /// The suspended element with `id`, if it's on the stage.
    pub fn find_suspended(&self, id: SuspendedId) -> Option<Rc<SuspendedWindow>> {
        self.stage
            .windows()
            .filter_map(|w| w.suspended())
            .find(|s| s.id == id)
            .cloned()
    }

    /// Focus + raise a suspended window (its body was clicked/tapped). Focus is
    /// intent-only: a suspended window holds no seat keyboard focus.
    pub fn focus_and_raise_suspended(&mut self, id: SuspendedId) {
        let Some(s) = self.find_suspended(id) else {
            return;
        };
        let element = StageWindow::Suspended(s);
        self.stage.raise_with_children(&element);
        self.enforce_below_windows();
        let serial = SERIAL_COUNTER.next_serial();
        self.set_suspended_focus(id, serial);
    }

    /// Relaunch the app behind a suspended window. Stub: chunk 5 mints the
    /// activation token, spawns via the resolved `Exec=`, and drives adoption.
    pub fn relaunch_suspended(&mut self, id: SuspendedId) {
        if self.find_suspended(id).is_none() {
            return;
        }
        tracing::info!("relaunch of suspended window {id:?} requested (not yet wired)");
    }

    /// Whether a suspended window is mid-relaunch, for the "launching…" label.
    /// Stub: chunk 5 tracks pending relaunches; nothing is pending yet.
    pub fn is_suspended_launching(&self, _id: SuspendedId) -> bool {
        false
    }

    /// Dismiss (close) a suspended window: drop it from the stage and its chrome
    /// caches, then run the same focus-follow a real window close does.
    pub fn dismiss_suspended(&mut self, id: SuspendedId) {
        let Some(s) = self.find_suspended(id) else {
            return;
        };
        let was_focused = matches!(
            self.window_focus,
            Some(crate::state::FocusIntent::Suspended(sid)) if sid == id
        );

        self.stage.remove(&StageWindow::Suspended(s));
        self.decorations.remove(&DecorationKey::Suspended(id));
        self.render
            .border_cache
            .remove(&DecorationKey::Suspended(id));
        self.render
            .shadow_cache
            .remove(&DecorationKey::Suspended(id));

        if was_focused {
            // Close-style follow: return to the most-recent live window, panning
            // only if it isn't already fully on screen.
            let follow = self
                .stage
                .focus_history()
                .iter()
                .filter_map(|w| w.client())
                .find(|w| w.alive())
                .cloned();
            let serial = SERIAL_COUNTER.next_serial();
            match follow {
                Some(target) if self.window_fully_in_viewport(&target) => {
                    self.raise_and_focus(&target, serial);
                }
                Some(target) => self.navigate_to_window(&target, false),
                None => self.set_window_focus(None, serial),
            }
        }
        // The suspended window may have sat under the cursor; re-target so a
        // click no longer lands in dead space.
        self.refresh_pointer_focus();
    }
}

#[cfg(test)]
impl DriftWm {
    /// Materialize a suspended window at `pos` (content top-left) sized `size`,
    /// raised to the top of the z-order. Production never constructs a suspended
    /// element this way — chunk 4 owns conversion — so this exists only to
    /// exercise rendering, hit-testing, and focus in isolation.
    pub fn insert_suspended_for_test(
        &mut self,
        id: u64,
        pos: smithay::utils::Point<i32, smithay::utils::Logical>,
        size: smithay::utils::Size<i32, smithay::utils::Logical>,
        app_id: &str,
        display_name: &str,
    ) -> SuspendedId {
        let sid = SuspendedId(id);
        let identity = driftwm::desktop_entry::AppIdentity {
            app_id: app_id.to_string(),
            desktop_id: app_id.to_string(),
            display_name: display_name.to_string(),
        };
        let s = Rc::new(SuspendedWindow::new(
            sid,
            size,
            identity,
            display_name.to_string(),
        ));
        self.map_window(StageWindow::Suspended(s), pos, true);
        sid
    }
}
