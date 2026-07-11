//! Smithay-free source of truth for window state: the window list, z-order,
//! per-window canvas position, focus history / MRU cycle, fullscreen
//! membership, pin-to-screen membership, and fit state. `DriftWm` wrapper
//! methods (`map_window` / `raise_window` / `unmap_window`) are the only way
//! to mutate it, and a debug end-of-tick check asserts its invariants.
//!
//! The stage never touches protocol state (configures, buffers, damage). It
//! answers queries and records decisions; the compositor applies the effects
//! (keyboard focus, camera moves, configure sends).

mod element;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub use element::StageElement;

use smithay::utils::{Logical, Point, Size};
use std::collections::BTreeMap;

/// Stable per-window handle, assigned once when a window is first mapped.
/// Survives position/z-order changes; never reused within a session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ElementId(pub u64);

/// Fullscreen membership plus the pre-fullscreen geometry restored on exit.
/// The viewport half of fullscreen state (saved camera/zoom) stays on
/// `DriftWm`'s per-output state — cameras are not stage domain.
pub struct FullscreenEntry<W> {
    pub window: W,
    pub saved_location: Point<i32, Logical>,
    pub saved_size: Size<i32, Logical>,
}

/// Screen-space pin site for a window pinned to one output (the
/// `pinned_to_screen` rule or the pin toggle): output by name, plus the
/// output-relative top-left (Y-down, pre-zoom). Rendering, hit-testing, and
/// capture route through this instead of the camera transform.
#[derive(Clone, Debug, PartialEq)]
pub struct PinnedSite {
    pub output: String,
    pub screen_pos: Point<i32, Logical>,
}

struct Entry<W> {
    window: W,
    id: ElementId,
    position: Point<i32, Logical>,
    /// `Some(pre-fit size)` while the window is fit to the viewport.
    fit_saved_size: Option<Size<i32, Logical>>,
    /// The size fit/fullscreen restore to. Tracked separately from live
    /// geometry because some clients (Chromium) shrink their reported
    /// geometry after each sized configure.
    restore_size: Option<Size<i32, Logical>>,
    /// `Some` while the window is pinned to an output's screen space.
    pinned: Option<PinnedSite>,
}

pub struct Stage<W: StageElement> {
    /// Z-order, bottom → top (matches `Space::elements`).
    entries: Vec<Entry<W>>,
    /// MRU focus history, front = most recently focused.
    focus_history: Vec<W>,
    /// Index into `focus_history` while an Alt-Tab cycle is in progress.
    cycle_state: Option<usize>,
    /// Fullscreen window per output, keyed by output name.
    fullscreen: BTreeMap<String, FullscreenEntry<W>>,
    next_id: u64,
}

impl<W: StageElement> Default for Stage<W> {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            focus_history: Vec::new(),
            cycle_state: None,
            fullscreen: BTreeMap::new(),
            next_id: 0,
        }
    }
}

impl<W: StageElement> Stage<W> {
    pub fn new() -> Self {
        Self::default()
    }

    fn entry(&self, window: &W) -> Option<&Entry<W>> {
        self.entries.iter().find(|e| &e.window == window)
    }

    fn entry_mut(&mut self, window: &W) -> Option<&mut Entry<W>> {
        self.entries.iter_mut().find(|e| &e.window == window)
    }

    /// Insert `window` at the top of the z-order (or move it there) and set
    /// its position. Mirrors `Space::map_element`, which always raises.
    pub fn map(&mut self, window: W, position: Point<i32, Logical>) {
        if let Some(idx) = self.entries.iter().position(|e| e.window == window) {
            let mut entry = self.entries.remove(idx);
            entry.position = position;
            self.entries.push(entry);
        } else {
            let id = ElementId(self.next_id);
            self.next_id += 1;
            self.entries.push(Entry {
                window,
                id,
                position,
                fit_saved_size: None,
                restore_size: None,
                pinned: None,
            });
        }
    }

    /// Move an already-mapped window to the top of the z-order. No-op for
    /// unknown windows (mirrors `Space::raise_element`).
    pub fn raise(&mut self, window: &W) {
        if let Some(idx) = self.entries.iter().position(|e| &e.window == window) {
            let entry = self.entries.remove(idx);
            self.entries.push(entry);
        }
    }

    /// Remove a window everywhere: z-order, focus history (clamping any active
    /// cycle), and fullscreen.
    pub fn remove(&mut self, window: &W) {
        self.entries.retain(|e| &e.window != window);
        self.focus_history.retain(|w| w != window);
        self.clamp_cycle();
        self.fullscreen.retain(|_, fs| &fs.window != window);
    }

    /// Drop dead windows from the z-order. Mirrors `Space::refresh`'s element
    /// retention and, like it, leaves focus history untouched — dead history
    /// entries are purged by the destroy handlers.
    pub fn retain_alive(&mut self) {
        self.entries.retain(|e| e.window.is_alive());
    }

    /// Windows in z-order, bottom → top.
    pub fn windows(&self) -> impl DoubleEndedIterator<Item = &W> + ExactSizeIterator {
        self.entries.iter().map(|e| &e.window)
    }

    pub fn contains(&self, window: &W) -> bool {
        self.entry(window).is_some()
    }

    pub fn position_of(&self, window: &W) -> Option<Point<i32, Logical>> {
        self.entry(window).map(|e| e.position)
    }

    pub fn id_of(&self, window: &W) -> Option<ElementId> {
        self.entry(window).map(|e| e.id)
    }

    pub fn window_by_id(&self, id: ElementId) -> Option<&W> {
        self.entries.iter().find(|e| e.id == id).map(|e| &e.window)
    }

    /// Raise `window`, then its descendants breadth-first, so each child ends
    /// up directly above its own parent without jumping over unrelated windows
    /// higher in the stack. Returns the raise order so the caller can apply
    /// per-window side effects (activation).
    pub fn raise_with_children(&mut self, window: &W) -> Vec<W> {
        let stack: Vec<W> = self.entries.iter().map(|e| e.window.clone()).collect();
        let order = subtree_raise_order(&stack, window, |child, parent| child.is_child_of(parent));
        for w in &order {
            self.raise(w);
        }
        order
    }

    /// Re-assert stacking classes: every non-widget window is raised (in
    /// current relative order) above any widget, then fullscreen windows go on
    /// top.
    pub fn enforce_stacking(&mut self) {
        let raised: Vec<W> = self
            .entries
            .iter()
            .filter(|e| !e.window.is_widget())
            .map(|e| e.window.clone())
            .collect();
        for w in &raised {
            self.raise(w);
        }
        let fullscreen: Vec<W> = self
            .fullscreen
            .values()
            .map(|fs| fs.window.clone())
            .collect();
        for w in &fullscreen {
            self.raise(w);
        }
    }

    /// Move `window` to the front of the MRU history. Eligibility (widgets,
    /// pinned, modals stay out) is the caller's business.
    pub fn push_focus(&mut self, window: &W) {
        self.focus_history.retain(|w| w != window);
        self.focus_history.insert(0, window.clone());
    }

    pub fn focus_history(&self) -> &[W] {
        &self.focus_history
    }

    /// Remove `window` from the focus history, clamping any active cycle.
    /// Used by paths that exclude a still-live window from the cycle
    /// (pinning, widget rules).
    pub fn drop_from_focus_history(&mut self, window: &W) {
        self.focus_history.retain(|w| w != window);
        self.clamp_cycle();
    }

    /// Crash-path removal from the focus history by predicate (the surface is
    /// gone, so the caller can't always name the `W`). Clamps the cycle.
    pub fn remove_from_history_matching(&mut self, pred: impl Fn(&W) -> bool) {
        self.focus_history.retain(|w| !pred(w));
        self.clamp_cycle();
    }

    pub fn cycle_state(&self) -> Option<usize> {
        self.cycle_state
    }

    pub fn cancel_cycle(&mut self) {
        self.cycle_state = None;
    }

    /// Advance the MRU cycle one step (first step jumps to the previous
    /// window), skipping fullscreen windows. Returns the window to navigate
    /// to, or `None` when the history is empty or all-fullscreen.
    pub fn cycle_step(&mut self, backward: bool) -> Option<W> {
        if self.focus_history.is_empty() {
            return None;
        }
        let len = self.focus_history.len();
        let step = |i: usize| {
            if backward {
                (i + len - 1) % len
            } else {
                (i + 1) % len
            }
        };
        let mut idx = match self.cycle_state {
            Some(cur) => step(cur),
            None => 1 % len,
        };
        // Bounded by `len` so an all-fullscreen history can't loop.
        let mut steps = 0;
        while steps < len
            && self
                .focus_history
                .get(idx)
                .is_some_and(|w| self.is_fullscreen(w))
        {
            idx = step(idx);
            steps += 1;
        }
        let window = self
            .focus_history
            .get(idx)
            .filter(|w| !self.is_fullscreen(w))
            .cloned()?;
        self.cycle_state = Some(idx);
        Some(window)
    }

    /// End an Alt-Tab cycle: commit the selected window to the front of the
    /// history.
    pub fn end_cycle(&mut self) {
        let idx = self.cycle_state.take();
        if let Some(idx) = idx
            && let Some(window) = self.focus_history.get(idx).cloned()
        {
            self.focus_history.retain(|w| w != &window);
            self.focus_history.insert(0, window);
        }
    }

    fn clamp_cycle(&mut self) {
        if self.cycle_state.is_some() {
            if self.focus_history.is_empty() {
                self.cycle_state = None;
            } else if let Some(idx) = self.cycle_state.as_mut() {
                *idx = (*idx).min(self.focus_history.len() - 1);
            }
        }
    }

    pub fn set_fullscreen(
        &mut self,
        output: &str,
        window: W,
        saved_location: Point<i32, Logical>,
        saved_size: Size<i32, Logical>,
    ) {
        self.fullscreen.insert(
            output.to_owned(),
            FullscreenEntry {
                window,
                saved_location,
                saved_size,
            },
        );
    }

    pub fn take_fullscreen(&mut self, output: &str) -> Option<FullscreenEntry<W>> {
        self.fullscreen.remove(output)
    }

    pub fn fullscreen_on(&self, output: &str) -> Option<&FullscreenEntry<W>> {
        self.fullscreen.get(output)
    }

    pub fn fullscreen_entries(&self) -> impl Iterator<Item = (&String, &FullscreenEntry<W>)> {
        self.fullscreen.iter()
    }

    pub fn has_fullscreen(&self) -> bool {
        !self.fullscreen.is_empty()
    }

    pub fn is_fullscreen(&self, window: &W) -> bool {
        self.fullscreen.values().any(|fs| &fs.window == window)
    }

    pub fn fullscreen_output_of(&self, window: &W) -> Option<&str> {
        self.fullscreen
            .iter()
            .find(|(_, fs)| &fs.window == window)
            .map(|(name, _)| name.as_str())
    }

    /// Mark `window` fit, saving its pre-fit size for the eventual unfit.
    pub fn set_fit(&mut self, window: &W, saved_size: Size<i32, Logical>) {
        if let Some(e) = self.entry_mut(window) {
            e.fit_saved_size = Some(saved_size);
        }
    }

    pub fn is_fit(&self, window: &W) -> bool {
        self.entry(window)
            .is_some_and(|e| e.fit_saved_size.is_some())
    }

    pub fn fit_saved_size(&self, window: &W) -> Option<Size<i32, Logical>> {
        self.entry(window).and_then(|e| e.fit_saved_size)
    }

    /// Clear fit state, returning the saved pre-fit size (the unfit path).
    pub fn take_fit_saved_size(&mut self, window: &W) -> Option<Size<i32, Logical>> {
        self.entry_mut(window).and_then(|e| e.fit_saved_size.take())
    }

    pub fn clear_fit(&mut self, window: &W) {
        if let Some(e) = self.entry_mut(window) {
            e.fit_saved_size = None;
        }
    }

    pub fn restore_size(&self, window: &W) -> Option<Size<i32, Logical>> {
        self.entry(window).and_then(|e| e.restore_size)
    }

    pub fn set_restore_size(&mut self, window: &W, size: Size<i32, Logical>) {
        if let Some(e) = self.entry_mut(window) {
            e.restore_size = Some(size);
        }
    }

    pub fn set_restore_size_if_missing(&mut self, window: &W, size: Size<i32, Logical>) {
        if let Some(e) = self.entry_mut(window)
            && e.restore_size.is_none()
        {
            e.restore_size = Some(size);
        }
    }

    pub fn set_pin(&mut self, window: &W, site: PinnedSite) {
        if let Some(e) = self.entry_mut(window) {
            e.pinned = Some(site);
        }
    }

    pub fn take_pin(&mut self, window: &W) -> Option<PinnedSite> {
        self.entry_mut(window).and_then(|e| e.pinned.take())
    }

    pub fn pin_of(&self, window: &W) -> Option<&PinnedSite> {
        self.entry(window).and_then(|e| e.pinned.as_ref())
    }

    pub fn is_pinned(&self, window: &W) -> bool {
        self.entry(window).is_some_and(|e| e.pinned.is_some())
    }

    pub fn has_pinned(&self) -> bool {
        self.entries.iter().any(|e| e.pinned.is_some())
    }

    /// Pinned windows with their sites, in z-order bottom → top.
    pub fn pinned_windows(&self) -> impl Iterator<Item = (&W, &PinnedSite)> {
        self.entries
            .iter()
            .filter_map(|e| e.pinned.as_ref().map(|site| (&e.window, site)))
    }

    /// Assert structural invariants. Panics on violation — fix the stage bug,
    /// never lower the invariant. Membership checks are scoped to live
    /// windows: a dead window may linger in the focus history until its
    /// destroy handler runs.
    pub fn verify_invariants(&self) {
        for (i, e) in self.entries.iter().enumerate() {
            assert!(
                !self.entries[i + 1..].iter().any(|o| o.window == e.window),
                "duplicate window in z-order"
            );
            assert!(
                !self.entries[i + 1..].iter().any(|o| o.id == e.id),
                "duplicate element id"
            );
        }

        for (i, w) in self.focus_history.iter().enumerate() {
            assert!(
                !self.focus_history[i + 1..].contains(w),
                "duplicate window in focus history"
            );
            assert!(
                !w.is_alive() || self.contains(w),
                "live focus-history window missing from window list"
            );
        }

        if let Some(idx) = self.cycle_state {
            assert!(
                idx < self.focus_history.len(),
                "cycle index {idx} out of bounds (history len {})",
                self.focus_history.len()
            );
        }

        let mut seen_fullscreen: Vec<&W> = Vec::new();
        for (output, fs) in &self.fullscreen {
            assert!(
                !fs.window.is_alive() || self.contains(&fs.window),
                "live fullscreen window on {output} missing from window list"
            );
            assert!(
                fs.saved_size.w > 0 && fs.saved_size.h > 0,
                "fullscreen window on {output} has empty saved geometry"
            );
            assert!(
                !seen_fullscreen.contains(&&fs.window),
                "window fullscreen on more than one output"
            );
            seen_fullscreen.push(&fs.window);
        }

        for e in &self.entries {
            if let Some(saved) = e.fit_saved_size {
                assert!(
                    saved.w > 0 && saved.h > 0,
                    "fit window has empty saved size"
                );
            }
            if e.pinned.is_some() {
                assert!(
                    !self.focus_history.contains(&e.window),
                    "pinned window in focus history"
                );
                assert!(
                    !seen_fullscreen.contains(&&e.window),
                    "window both pinned and fullscreen"
                );
            }
        }
    }
}

/// Order in which to raise `target` and its descendants so each child ends up
/// directly above its own parent: `target` first, then descendants breadth-first,
/// leaving unrelated windows below the subtree untouched. `is_child(a, b)` reports
/// whether `a`'s parent is `b`. Already-visited windows are skipped, so cyclic
/// parent links still terminate.
pub fn subtree_raise_order<T>(stack: &[T], target: &T, is_child: impl Fn(&T, &T) -> bool) -> Vec<T>
where
    T: Clone + PartialEq,
{
    let mut order = vec![target.clone()];
    let mut i = 0;
    while i < order.len() {
        let parent = order[i].clone();
        for w in stack {
            if !order.contains(w) && is_child(w, &parent) {
                order.push(w.clone());
            }
        }
        i += 1;
    }
    order
}
