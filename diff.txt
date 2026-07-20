```diff
diff --git a/config.reference.toml b/config.reference.toml
index 83f3f8a..8416941 100644
--- a/config.reference.toml
+++ b/config.reference.toml
@@ -99,7 +99,7 @@
 # mouse_speed = 1.0           # mouse (drag) pan multiplier (1.0 = direct)
 # touch_speed = 1.0           # touchscreen gesture pan speed multiplier
 # drift = 0.5                 # momentum coast: 0 = off, 0.5 = default, 1 = floatiest
-# animation_speed = 0.3       # camera lerp factor (higher = faster)
+# animation_speed = 0.3       # camera/window animation speed (higher = faster)
 # auto_navigate_on_close = true  # on close, pan to the newly focused window if off-screen
                               # false = camera stays put; focus only moves to a visible window
 # auto_navigate_on_click = false  # completed click on a partially off-screen window also pans it in
@@ -287,6 +287,7 @@
 # #   toggle-fullscreen       — toggle focused window fullscreen
 # #   fit-window              — toggle maximize: centers + resets zoom + fills viewport; restore only resizes back
 # #   fit-window-snapped      — fit-window for the focused window's whole snap cluster
+# #   fill-window             — grow in place to fill free space; edges outside the usable area or overlapping another window pull back to a gap; press again to restore
 # #   toggle-pin-to-screen    — pin/unpin the focused window to the screen (ignores pan/zoom, floats above)
 # #   reload-config           — hot-reload config file
 # #   toggle-cursor-pan       — toggle cursor edge-pan (see [navigation.edge_pan])
@@ -354,6 +355,8 @@
 # # "ctrl+Print" = "spawn driftwm msg screenshot window -o - | wl-copy"
 # # Example: tap binding — bare modifier chord, fires on release with no key on top
 # # "alt+shift" = "switch-layout next"
+# # Example: fill-window (unbound by default)
+# # "mod+g" = "fill-window"
 
 [mouse]
 # # When true (default), dragging a window's edge or corner resizes it via the
@@ -432,7 +435,7 @@
 # #   N-finger-hold             — threshold only (fires on release)
 # #
 # # Continuous actions: pan-viewport, zoom, move-window, move-snapped-windows, resize-window, resize-window-snapped
-# # Threshold actions: center-nearest, center-window, home-toggle, zoom-to-fit, zoom-to-fit-snapped, fit-window, fit-window-snapped, exec <cmd>, etc.
+# # Threshold actions: center-nearest, center-window, home-toggle, zoom-to-fit, zoom-to-fit-snapped, fit-window, fit-window-snapped, fill-window, exec <cmd>, etc.
 
 [gestures.on-window]
 # "alt+3-finger-swipe" = "resize-window"
@@ -489,7 +492,7 @@
 # # Continuous actions: pan-viewport (swipe), zoom (pinch), and the window grabs —
 # # move-window / move-snapped-windows / resize-window / resize-window-snapped
 # # (doubletap-swipe / hold-swipe). A held move-window also extends to the snap-cluster.
-# # Threshold actions: center-nearest, center-window, home-toggle, zoom-to-fit, fit-window, exec <cmd>, etc.
+# # Threshold actions: center-nearest, center-window, home-toggle, zoom-to-fit, fit-window, fill-window, exec <cmd>, etc.
 # #
 # # Note: within one physical gesture, a continuous translation (pan) and a
 # # threshold pinch on the same finger count don't combine — either bind both axes
diff --git a/docs/config.md b/docs/config.md
index 61e650c..6ea5e40 100644
--- a/docs/config.md
+++ b/docs/config.md
@@ -264,7 +264,7 @@ momentum coast: 0 = off, 0.5 = default, 1 = floatiest
 
 Default: `0.3`
 
-camera lerp factor (higher = faster)
+camera/window animation speed (higher = faster)
 
 ### `auto_navigate_on_close`
 
@@ -638,6 +638,7 @@ Actions:
 - `toggle-fullscreen` — toggle focused window fullscreen
 - `fit-window` — toggle maximize: centers + resets zoom + fills viewport; restore only resizes back
 - `fit-window-snapped` — fit-window for the focused window's whole snap cluster
+- `fill-window` — grow in place to fill free space; edges outside the usable area or overlapping another window pull back to a gap; press again to restore
 - `toggle-pin-to-screen` — pin/unpin the focused window to the screen (ignores pan/zoom, floats above)
 - `reload-config` — hot-reload config file
 - `toggle-cursor-pan` — toggle cursor edge-pan (see [navigation.edge_pan])
@@ -717,6 +718,12 @@ Directions: up, down, left, right, up-left, up-right, down-left, down-right
 "alt+shift" = "switch-layout next"
 ```
 
+**Example: fill-window (unbound by default)**
+
+```toml
+"mod+g" = "fill-window"
+```
+
 ## `[mouse]`
 
 ### `resize_on_border`
@@ -805,7 +812,7 @@ Gesture types:
 - `N-finger-pinch-in/out` — threshold only
 - `N-finger-hold` — threshold only (fires on release)
 
-Continuous actions: pan-viewport, zoom, move-window, move-snapped-windows, resize-window, resize-window-snapped Threshold actions: center-nearest, center-window, home-toggle, zoom-to-fit, zoom-to-fit-snapped, fit-window, fit-window-snapped, exec <cmd>, etc.
+Continuous actions: pan-viewport, zoom, move-window, move-snapped-windows, resize-window, resize-window-snapped Threshold actions: center-nearest, center-window, home-toggle, zoom-to-fit, zoom-to-fit-snapped, fit-window, fit-window-snapped, fill-window, exec <cmd>, etc.
 
 ## `[gestures.on-window]`
 
@@ -871,7 +878,7 @@ Touch gesture types (1–5 fingers):
 - `N-finger-doubletap-swipe` — continuous only (tap then drag)
 - `N-finger-hold-swipe` — continuous only (dwell then drag)
 
-Continuous actions: pan-viewport (swipe), zoom (pinch), and the window grabs — move-window / move-snapped-windows / resize-window / resize-window-snapped (doubletap-swipe / hold-swipe). A held move-window also extends to the snap-cluster. Threshold actions: center-nearest, center-window, home-toggle, zoom-to-fit, fit-window, exec <cmd>, etc.
+Continuous actions: pan-viewport (swipe), zoom (pinch), and the window grabs — move-window / move-snapped-windows / resize-window / resize-window-snapped (doubletap-swipe / hold-swipe). A held move-window also extends to the snap-cluster. Threshold actions: center-nearest, center-window, home-toggle, zoom-to-fit, fit-window, fill-window, exec <cmd>, etc.
 
 Note: within one physical gesture, a continuous translation (pan) and a threshold pinch on the same finger count don't combine — either bind both axes continuous (pan + zoom) or drive discrete actions from a threshold swipe/pinch.
 
diff --git a/proptest-regressions/layout/fill.txt b/proptest-regressions/layout/fill.txt
new file mode 100644
index 0000000..f8d2aed
--- /dev/null
+++ b/proptest-regressions/layout/fill.txt
@@ -0,0 +1,7 @@
+# Seeds for failure cases proptest has generated in the past. It is
+# automatically read and these particular cases re-run before any
+# novel cases are generated.
+#
+# It is recommended to check this file in to source control so that
+# everyone who runs the test benefits from these saved cases.
+cc ae930fdf3bb92798a00dfff997229247493c87d255703fb1e8d43315757dceac # shrinks to current = SnapRect { x_low: -353.5049533833128, x_high: 180.04742708087497, y_low: 0.0, y_high: 20.0 }, obstacles = [SnapRect { x_low: -246.36573871237303, x_high: 279.0959729852267, y_low: 0.0, y_high: 591.9888807208005 }], gap = 19.404587836129505
diff --git a/src/backend/udev.rs b/src/backend/udev.rs
index a8705f3..4745754 100644
--- a/src/backend/udev.rs
+++ b/src/backend/udev.rs
@@ -220,6 +220,16 @@ pub(crate) fn render_if_needed(data: &mut DriftWm) {
         return;
     };
 
+    // Remember what was active before ticking. An animation may finish during
+    // this tick, but its final state still needs to be presented once.
+    let animated_outputs_before: Vec<Output> = data
+        .space
+        .outputs()
+        .filter(|output| data.output_has_active_animations(output))
+        .cloned()
+        .collect();
+    let global_visual_animation_before = data.has_global_visual_animations();
+
     // 1. Tick animations once for all outputs (before device borrow)
     data.tick_all_animations();
 
@@ -270,7 +280,9 @@ pub(crate) fn render_if_needed(data: &mut DriftWm) {
         if data.dpms_off_outputs.contains(&surface.output) {
             continue;
         }
-        if data.output_has_active_animations(&surface.output) {
+        if animated_outputs_before.contains(&surface.output)
+            || data.output_has_active_animations(&surface.output)
+        {
             data.redraws_needed.insert(surface.output.clone());
         }
         // Chunked-bg with tiles still to upload: keep firing frames until the
@@ -295,6 +307,8 @@ pub(crate) fn render_if_needed(data: &mut DriftWm) {
         || data.cursor.exec_cursor_show_at.is_some()
         || data.cursor.exec_cursor_deadline.is_some()
         || data.cursor_is_animated()
+        || global_visual_animation_before
+        || data.has_global_visual_animations()
     {
         data.mark_all_dirty();
     } else if data.render.background_is_animated {
@@ -660,6 +674,7 @@ pub fn init_udev(
         .handle()
         .insert_source(drm_notifier, move |event, meta, data: &mut DriftWm| {
             let mut dev = device_for_drm.borrow_mut();
+            let mut render_after_event = false;
             match event {
                 DrmEvent::VBlank(crtc) => {
                     let Some(surface) = dev.surfaces.get_mut(&crtc) else {
@@ -677,14 +692,20 @@ pub fn init_udev(
                     if let Some(token) = data.estimated_vblank_timers.remove(&crtc) {
                         data.loop_handle.remove(token);
                     }
-                    if data.redraws_needed.contains(&surface.output) {
-                        render_frame(data, &mut surface.compositor, &surface.output, crtc);
-                    }
+                    render_after_event = true;
                 }
                 DrmEvent::Error(err) => {
                     tracing::error!("DRM error: {err}");
                 }
             }
+            drop(dev);
+
+            // VBlank is the clock for output animations. Re-enter the regular
+            // render path so it advances animation state and marks the affected
+            // output dirty before deciding whether another frame is needed.
+            if render_after_event {
+                render_if_needed(data);
+            }
         })?;
 
     // 10. Register session notifier (VT switching)
@@ -1517,6 +1538,10 @@ fn queue_estimated_vblank_timer(data: &mut DriftWm, output: &Output, crtc: crtc:
         .loop_handle
         .insert_source(timer, move |_, _, data: &mut DriftWm| {
             data.estimated_vblank_timers.remove(&crtc);
+            // EmptyFrame produces no kernel VBlank. Treat this timer as its
+            // replacement so an ongoing animation can advance and request the
+            // next frame without waiting for unrelated input or client damage.
+            render_if_needed(data);
             TimeoutAction::Drop
         }) {
         Ok(tok) => {
diff --git a/src/backend/winit.rs b/src/backend/winit.rs
index 6bb3e9d..ff8967a 100644
--- a/src/backend/winit.rs
+++ b/src/backend/winit.rs
@@ -173,6 +173,9 @@ pub fn init_winit(
             // --- Camera animation (window navigation) ---
             data.apply_camera_animation(dt);
 
+            // --- Window open/close/move/resize animations ---
+            data.tick_window_animations(dt);
+
             // --- Coalesced pointer motion (after input + animations) ---
             data.flush_pointer_resync();
 
diff --git a/src/config/mod.rs b/src/config/mod.rs
index 657a7b8..0e84976 100644
--- a/src/config/mod.rs
+++ b/src/config/mod.rs
@@ -70,8 +70,8 @@ pub struct Config {
     pub edge_pan_cursor: bool,
     /// Cursor edge-pan activation zone, px from the edge.
     pub edge_pan_cursor_zone: f64,
-    /// Base lerp factor for camera animation (frame-rate independent), in (0, 1].
-    /// Lower = smoother; 1 = instant; 0 would freeze the camera.
+    /// Base lerp factor for camera and window animations (frame-rate
+    /// independent), in (0, 1]. Lower = smoother; 1 = instant.
     pub animation_speed: f64,
     /// On close, pan the camera to the newly focused window (true). When false,
     /// focus only moves to an already-visible window — never off-screen.
@@ -725,13 +725,12 @@ impl Config {
             "navigation.drift",
             &mut errors,
         );
-        // Valid range is (0, 1]: at 0 the lerp factor stays 0 and the camera
-        // never reaches its target, so reject it (and negatives/NaN) back to the
-        // default rather than freezing. Above 1 just clamps to instant.
+        // Valid range is (0, 1]: at 0 the lerp factor stays 0 and animations
+        // never reach their targets. Above 1 just clamps to instant.
         let animation_speed = match raw.navigation.animation_speed {
             Some(v) if v <= 0.0 || v.is_nan() => {
                 warn_and_collect!(
-                    "config: navigation.animation_speed {v} must be in (0, 1] (0 freezes the camera), using 0.3"
+                    "config: navigation.animation_speed {v} must be in (0, 1] (0 freezes animations), using 0.3"
                 );
                 0.3
             }
diff --git a/src/config/parse.rs b/src/config/parse.rs
index 2cc3d5e..439f93d 100644
--- a/src/config/parse.rs
+++ b/src/config/parse.rs
@@ -160,6 +160,7 @@ pub fn parse_action(s: &str) -> Result<Action, String> {
         "toggle-fullscreen" => Ok(Action::ToggleFullscreen),
         "fit-window" => Ok(Action::FitWindow),
         "fit-window-snapped" => Ok(Action::FitWindowSnapped),
+        "fill-window" => Ok(Action::FillWindow),
         "send-to-output" => {
             let dir = parse_direction(arg.ok_or("send-to-output requires a direction")?)?;
             Ok(Action::SendToOutput(dir))
@@ -321,6 +322,7 @@ fn parse_threshold_action(s: &str) -> Result<Option<ThresholdAction>, String> {
         | "toggle-fullscreen"
         | "fit-window"
         | "fit-window-snapped"
+        | "fill-window"
         | "toggle-pin-to-screen"
         | "reload-config"
         | "toggle-cursor-pan"
diff --git a/src/config/types.rs b/src/config/types.rs
index b5f88f2..4681294 100644
--- a/src/config/types.rs
+++ b/src/config/types.rs
@@ -67,6 +67,7 @@ pub enum Action {
     ToggleFullscreen,
     FitWindow,
     FitWindowSnapped,
+    FillWindow,
     SendToOutput(Direction),
     SendCursorToOutput(Direction),
     FocusCenter,
diff --git a/src/grabs/touch_gesture_grab.rs b/src/grabs/touch_gesture_grab.rs
index d6e6431..400645c 100644
--- a/src/grabs/touch_gesture_grab.rs
+++ b/src/grabs/touch_gesture_grab.rs
@@ -219,12 +219,19 @@ impl TouchGestureGrab {
         }
         let serial = SERIAL_COUNTER.next_serial();
         data.raise_and_focus(&window, serial);
+        // Moving re-anchors the window, invalidating any fill restore point.
+        data.stage.clear_fill(&window);
         let initial = data.stage.position_of(&window).unwrap_or(loc);
         let (members, surfaces) = if cluster {
             data.cluster_snapshot_for_drag(&window, initial)
         } else {
             (Vec::new(), HashSet::new())
         };
+        // Members ride along with the primary, so their fill restore points go
+        // stale too.
+        for (member, _) in &members {
+            data.stage.clear_fill(member);
+        }
         let start = TouchGrabStartData {
             focus: None,
             slot: event.slot,
diff --git a/src/handlers/compositor.rs b/src/handlers/compositor.rs
index b0991c9..b9a07cf 100644
--- a/src/handlers/compositor.rs
+++ b/src/handlers/compositor.rs
@@ -576,6 +576,7 @@ impl CompositorHandler for DriftWm {
                         // `fit_window_snapped` overwrites with the post-fit
                         // rect; non-snapped fit and fullscreen keep this.
                         self.refresh_stable_snap_rect(&window);
+                        self.start_window_open_animation(&window);
 
                         if let Some(client_output) = self.pending_fullscreen.remove(&root) {
                             let target = self.resolve_fullscreen_output(&root, client_output);
@@ -896,7 +897,13 @@ impl DriftWm {
         if !matches!(resize_state, ResizeState::Idle) {
             return;
         }
-        if self.is_window_fullscreen(window) || self.stage.is_fit(window) {
+        // A filled window is deliberately grown in place and may retain an
+        // unresolvable overlap; reflowing it here would translate it (violating
+        // fill's never-move contract) off a now-stale stable snap rect.
+        if self.is_window_fullscreen(window)
+            || self.stage.is_fit(window)
+            || self.stage.is_fill(window)
+        {
             return;
         }
 
diff --git a/src/handlers/mod.rs b/src/handlers/mod.rs
index 0441e90..b148248 100644
--- a/src/handlers/mod.rs
+++ b/src/handlers/mod.rs
@@ -659,7 +659,7 @@ impl ForeignToplevelHandler for DriftWm {
     fn close(&mut self, wl_surface: WlSurface) {
         let window = self.window_for_surface(&wl_surface);
         if let Some(window) = window {
-            window.send_close();
+            self.request_window_close(&window);
         }
     }
 
@@ -1023,7 +1023,7 @@ impl SessionLockHandler for DriftWm {
             os.panning = false;
             os.camera_target = None;
             os.zoom_target = None;
-            os.zoom_animation_center = None;
+            os.zoom_animation_anchor = None;
         }
         self.held_action = None;
         self.cursor.grab_cursor = false;
diff --git a/src/handlers/xdg_shell.rs b/src/handlers/xdg_shell.rs
index 6e03592..aad7bb2 100644
--- a/src/handlers/xdg_shell.rs
+++ b/src/handlers/xdg_shell.rs
@@ -389,6 +389,8 @@ impl XdgShellHandler for DriftWm {
             let Some(output) = self.active_output() else {
                 return;
             };
+            // Moving re-anchors the window, invalidating any fill restore point.
+            self.stage.clear_fill(&window);
             let grab = MoveSurfaceGrab::new(
                 start_data,
                 window,
@@ -433,6 +435,8 @@ impl XdgShellHandler for DriftWm {
             // of the grab — so cancel the client's sequence before the compositor
             // takes over the drag, or it keeps receiving the whole sequence.
             touch.cancel(self);
+            // Moving re-anchors the window, invalidating any fill restore point.
+            self.stage.clear_fill(&window);
             let grab = MoveSurfaceGrab::new_touch(
                 touch_start,
                 window,
@@ -478,8 +482,9 @@ impl XdgShellHandler for DriftWm {
             return;
         };
 
-        // Clear fit state — user took manual control
+        // Clear fit/fill state — user took manual control
         self.stage.clear_fit(&window);
+        self.stage.clear_fill(&window);
 
         // Pinned windows resize in screen space (see start_compositor_resize_with_edge).
         let pinned_site = self.stage.pin_of(&window).cloned();
diff --git a/src/input/actions.rs b/src/input/actions.rs
index 0250d50..b5386da 100644
--- a/src/input/actions.rs
+++ b/src/input/actions.rs
@@ -1,6 +1,6 @@
 use smithay::{
-    input::{keyboard::Layout, pointer::MotionEvent},
-    utils::{Logical, Point, SERIAL_COUNTER, Size},
+    input::keyboard::Layout,
+    utils::{Logical, Point, Size},
     wayland::seat::WaylandFocus,
 };
 
@@ -50,13 +50,15 @@ impl DriftWm {
             }
             Action::CloseWindow => {
                 if let Some(window) = self.focused_window().filter(|w| !w.is_widget()) {
-                    window.send_close();
+                    self.request_window_close(&window);
                 }
             }
             Action::NudgeWindow(dir) => {
                 if let Some(window) = self.focused_window().filter(|w| self.is_canvas_window(w))
                     && let Some(loc) = self.stage.position_of(&window)
                 {
+                    // Nudging re-anchors the window, invalidating any fill restore point.
+                    self.stage.clear_fill(&window);
                     let step = self.config.nudge_step;
                     let (ux, uy) = dir.to_unit_vec();
                     let offset = (
@@ -64,14 +66,14 @@ impl DriftWm {
                         (uy * step as f64).round() as i32,
                     );
                     let new_loc = loc + Point::from(offset);
+                    self.animate_window_geometry(&window, new_loc, window.geometry().size);
                     self.map_window(window.clone(), new_loc, false);
                 }
             }
             Action::PanViewport(dir) => {
                 let Some(zoom) = self.with_output_state(|os| {
-                    os.camera_target = None;
                     os.zoom_target = None;
-                    os.zoom_animation_center = None;
+                    os.zoom_animation_anchor = None;
                     os.overview_return = None;
                     os.zoom
                 }) else {
@@ -81,25 +83,10 @@ impl DriftWm {
                 let (ux, uy) = dir.to_unit_vec();
                 let delta: Point<f64, smithay::utils::Logical> =
                     Point::from((ux * step, uy * step));
-                self.set_camera(self.camera() + delta);
-                self.update_output_from_camera();
-
-                // Shift pointer so cursor stays at the same screen position
-                let pointer = self.seat.get_pointer().unwrap();
-                let pos = pointer.current_location();
-                let new_pos = pos + delta;
-                let under = self.surface_under(new_pos, None);
-                let serial = SERIAL_COUNTER.next_serial();
-                pointer.motion(
-                    self,
-                    under,
-                    &MotionEvent {
-                        location: new_pos,
-                        serial,
-                        time: self.start_time.elapsed().as_millis() as u32,
-                    },
-                );
-                pointer.frame(self);
+                // Repeated key actions extend the destination instead of
+                // restarting from the partially animated camera position.
+                let target = self.camera_target().unwrap_or_else(|| self.camera()) + delta;
+                self.set_camera_target(Some(target));
             }
             Action::CenterWindow => {
                 if let Some(window) = self.focused_window().filter(|w| self.is_canvas_window(w)) {
@@ -253,10 +240,13 @@ impl DriftWm {
                             self.enter_fullscreen(ret.fullscreen_window.as_ref().unwrap(), None);
                         } else {
                             let vc = self.usable_center_screen();
-                            self.set_zoom_animation_center(Some(Point::from((
-                                ret.camera.x + vc.x / ret.zoom,
-                                ret.camera.y + vc.y / ret.zoom,
-                            ))));
+                            self.set_zoom_animation_anchor(
+                                Point::from((
+                                    ret.camera.x + vc.x / ret.zoom,
+                                    ret.camera.y + vc.y / ret.zoom,
+                                )),
+                                vc,
+                            );
                             self.set_camera_target(Some(ret.camera));
                             self.set_zoom_target(Some(ret.zoom));
                         }
@@ -281,7 +271,7 @@ impl DriftWm {
                         self.update_output_from_camera();
                         self.warp_pointer(Point::from((0.0, 0.0)));
                     } else {
-                        self.set_zoom_animation_center(Some(Point::from((0.0, 0.0))));
+                        self.set_zoom_animation_anchor(Point::from((0.0, 0.0)), vc);
                         self.set_camera_target(Some(home));
                         self.set_zoom_target(Some(1.0));
                     }
@@ -396,6 +386,11 @@ impl DriftWm {
                     self.toggle_fit_window_snapped(&window);
                 }
             }
+            Action::FillWindow => {
+                if let Some(window) = self.focused_window().filter(|w| self.is_canvas_window(w)) {
+                    self.toggle_fill_window(&window);
+                }
+            }
             Action::SendToOutput(dir) => {
                 let Some(window) = self.focused_window().filter(|w| !w.is_widget()) else {
                     return;
@@ -443,6 +438,9 @@ impl DriftWm {
                         (center_x - geo.size.w as f64 / 2.0) as i32,
                         (center_y - geo.size.h as f64 / 2.0) as i32,
                     ));
+                    // Relocating to another output re-anchors the window,
+                    // invalidating any fill restore point.
+                    self.stage.clear_fill(&window);
                     self.map_window(window.clone(), new_loc, true);
                     let serial = smithay::utils::SERIAL_COUNTER.next_serial();
                     self.raise_and_focus(&window, serial);
@@ -595,10 +593,13 @@ impl DriftWm {
         };
         self.set_overview_return(None);
         let vc = self.usable_center_screen();
-        self.set_zoom_animation_center(Some(Point::from((
-            saved_camera.x + vc.x / saved_zoom,
-            saved_camera.y + vc.y / saved_zoom,
-        ))));
+        self.set_zoom_animation_anchor(
+            Point::from((
+                saved_camera.x + vc.x / saved_zoom,
+                saved_camera.y + vc.y / saved_zoom,
+            )),
+            vc,
+        );
         self.set_camera_target(Some(saved_camera));
         self.set_zoom_target(Some(saved_zoom));
         true
@@ -616,7 +617,7 @@ impl DriftWm {
         let new_camera: Point<f64, smithay::utils::Logical> =
             Point::from((bbox_cx - vc.x / fit_zoom, bbox_cy - vc.y / fit_zoom));
         self.set_overview_return(Some((self.camera(), self.zoom())));
-        self.set_zoom_animation_center(Some(Point::from((bbox_cx, bbox_cy))));
+        self.set_zoom_animation_anchor(Point::from((bbox_cx, bbox_cy)), vc);
         self.set_camera_target(Some(new_camera));
         self.set_zoom_target(Some(fit_zoom));
     }
@@ -629,7 +630,7 @@ impl DriftWm {
         let zoom = self.zoom();
         let vc_canvas = Point::from((camera.x + vc.x / zoom, camera.y + vc.y / zoom));
         let new_camera = canvas::zoom_anchor_camera(vc_canvas, vc, target_zoom);
-        self.set_zoom_animation_center(Some(vc_canvas));
+        self.set_zoom_animation_anchor(vc_canvas, vc);
         self.set_zoom_target(Some(target_zoom));
         self.set_camera_target(Some(new_camera));
     }
diff --git a/src/input/gestures.rs b/src/input/gestures.rs
index 087be9b..2ebea9a 100644
--- a/src/input/gestures.rs
+++ b/src/input/gestures.rs
@@ -64,7 +64,7 @@ impl DriftWm {
         self.with_output_state(|os| {
             os.camera_target = None;
             os.zoom_target = None;
-            os.zoom_animation_center = None;
+            os.zoom_animation_anchor = None;
             os.momentum.stop();
         });
     }
diff --git a/src/input/gestures/swipe.rs b/src/input/gestures/swipe.rs
index cabb6a8..fb21cfd 100644
--- a/src/input/gestures/swipe.rs
+++ b/src/input/gestures/swipe.rs
@@ -458,6 +458,12 @@ impl DriftWm {
         let Some(output) = self.active_output() else {
             return;
         };
+        // Moving re-anchors the window, invalidating any fill restore point —
+        // for the primary and every member dragged along.
+        self.stage.clear_fill(&window);
+        for (member, _) in &members {
+            self.stage.clear_fill(member);
+        }
         let grab = MoveSurfaceGrab::new(
             GrabStartData {
                 focus: None,
@@ -518,8 +524,9 @@ impl DriftWm {
         let initial_size = window.geometry().size;
         let edges = edges_from_position(pos, initial_location, initial_size);
 
-        // Clear fit state — user took manual control
+        // Clear fit/fill state — user took manual control
         self.stage.clear_fit(&window);
+        self.stage.clear_fill(&window);
 
         // Store resize state on surface data map for commit() repositioning
         with_states(&wl_surface, |states| {
diff --git a/src/input/pointer.rs b/src/input/pointer.rs
index 9e8519f..6f73e21 100644
--- a/src/input/pointer.rs
+++ b/src/input/pointer.rs
@@ -22,7 +22,9 @@ use smithay::wayland::seat::WaylandFocus;
 
 use crate::decorations::DecorationHit;
 use crate::grabs::{MoveSurfaceGrab, NavigateGrab, PanGrab, ResizeState, ResizeSurfaceGrab};
-use crate::state::{ClusterResizeSnapshot, DriftWm, FocusTarget, PendingMiddleClick};
+use crate::state::{
+    ClusterResizeSnapshot, DriftWm, FocusTarget, PendingMiddleClick, ZoomAnimationAnchor,
+};
 use driftwm::canvas::{self, CanvasPos, canvas_to_screen};
 use driftwm::config::{self, BindingContext, MouseAction};
 use driftwm::window_ext::WindowExt;
@@ -254,7 +256,7 @@ impl DriftWm {
                     if button == config::BTN_LEFT {
                         match hit {
                             DecorationHit::CloseButton => {
-                                window.send_close();
+                                self.request_window_close(&window);
                                 return;
                             }
                             DecorationHit::TitleBar if !is_widget => {
@@ -294,6 +296,9 @@ impl DriftWm {
                                     button,
                                     location: pos,
                                 };
+                                // Moving re-anchors the window, so a fill restore
+                                // point (which includes position) no longer applies.
+                                self.stage.clear_fill(&window);
                                 let grab = MoveSurfaceGrab::new(
                                     start_data,
                                     window,
@@ -362,6 +367,12 @@ impl DriftWm {
                             } else {
                                 (Vec::new(), HashSet::new())
                             };
+                            // Re-anchoring invalidates any fill restore point —
+                            // for the primary and every member dragged along.
+                            self.stage.clear_fill(&window);
+                            for (member, _) in &cluster_members {
+                                self.stage.clear_fill(member);
+                            }
                             let grab = MoveSurfaceGrab::new(
                                 start_data,
                                 window,
@@ -523,7 +534,7 @@ impl DriftWm {
                 .and_then(|s| config::applied_rule(&s))
                 .is_some_and(|r| r.widget);
             match hit {
-                DecorationHit::CloseButton => window.send_close(),
+                DecorationHit::CloseButton => self.request_window_close(&window),
                 DecorationHit::TitleBar if !is_widget => {
                     self.raise_and_focus(&window, serial);
                     self.start_pinned_move(pointer, &window, pos, button, serial);
@@ -712,8 +723,9 @@ impl DriftWm {
             return;
         };
 
-        // Clear fit state — user took manual control
+        // Clear fit/fill state — user took manual control
         self.stage.clear_fit(window);
+        self.stage.clear_fill(window);
 
         // Pinned windows resize in screen space; capture their `screen_pos` and
         // fixed output so the grab and the commit-time reposition use the right
@@ -933,22 +945,23 @@ impl DriftWm {
                         let steps = -v / 30.0 * self.config.zoom_mouse_speed;
                         let factor = self.config.zoom_step.powf(steps);
                         let cur_zoom = self.zoom();
-                        let new_zoom = (cur_zoom * factor).clamp(self.min_zoom(), canvas::MAX_ZOOM);
+                        let base_zoom = self.zoom_target().unwrap_or(cur_zoom);
+                        let target_zoom =
+                            (base_zoom * factor).clamp(self.min_zoom(), canvas::MAX_ZOOM);
 
-                        if new_zoom != cur_zoom {
+                        if target_zoom != base_zoom {
                             let screen_pos =
                                 canvas_to_screen(CanvasPos(pos), self.camera(), cur_zoom).0;
-                            let new_camera = canvas::zoom_anchor_camera(pos, screen_pos, new_zoom);
                             self.with_output_state(|os| {
-                                os.camera = new_camera;
-                                os.zoom = new_zoom;
-                                os.zoom_target = None;
-                                os.zoom_animation_center = None;
+                                os.zoom_target = Some(target_zoom);
+                                os.zoom_animation_anchor = Some(ZoomAnimationAnchor {
+                                    canvas: pos,
+                                    screen: screen_pos,
+                                });
                                 os.camera_target = None;
                                 os.overview_return = None;
                                 os.momentum.stop();
                             });
-                            self.update_output_from_camera();
 
                             let under = self.surface_under(pos, None);
                             let serial = SERIAL_COUNTER.next_serial();
diff --git a/src/input/touch.rs b/src/input/touch.rs
index 0419054..35f59e7 100644
--- a/src/input/touch.rs
+++ b/src/input/touch.rs
@@ -340,6 +340,7 @@ impl DriftWm {
             .unwrap_or(output);
 
         self.stage.clear_fit(window);
+        self.stage.clear_fill(window);
 
         with_states(&wl_surface, |states| {
             states
@@ -608,6 +609,8 @@ impl DriftWm {
             return;
         };
         self.raise_and_focus(window, serial);
+        // Moving re-anchors the window, invalidating any fill restore point.
+        self.stage.clear_fill(window);
         let start = TouchGrabStartData {
             focus: None,
             slot,
@@ -717,7 +720,7 @@ impl DriftWm {
                     )
                 };
                 if still_inside {
-                    pc.window.send_close();
+                    self.request_window_close(&pc.window);
                 }
                 return;
             }
diff --git a/src/ipc/mod.rs b/src/ipc/mod.rs
index 929546e..adde922 100644
--- a/src/ipc/mod.rs
+++ b/src/ipc/mod.rs
@@ -434,6 +434,8 @@ fn cmd_move(window: Option<WindowSelector>, to: Option<(i32, i32)>, state: &mut
                 return Err("pinned and fullscreen windows have no canvas position to move".into());
             }
             let loc = driftwm::canvas::rule_to_internal(x, y, size);
+            // Moving re-anchors the window, invalidating any fill restore point.
+            state.stage.clear_fill(&window);
             // Activating is only consistent when the target already holds
             // focus; a selector can reach any window.
             let activate = state.focused_window().as_ref() == Some(&window);
@@ -445,7 +447,7 @@ fn cmd_move(window: Option<WindowSelector>, to: Option<(i32, i32)>, state: &mut
 
 fn cmd_close(sel: Option<WindowSelector>, state: &mut DriftWm) -> Reply {
     let window = window_by_selector(state, sel.as_ref())?;
-    window.send_close();
+    state.request_window_close(&window);
     Ok(Response::Ok)
 }
 
diff --git a/src/layout/fill.rs b/src/layout/fill.rs
new file mode 100644
index 0000000..e810e95
--- /dev/null
+++ b/src/layout/fill.rs
@@ -0,0 +1,691 @@
+//! Geometry for `fill-window`: grow a window in place to fill the free space
+//! around it, stopping at neighboring windows and the usable-area edge (both
+//! with a snap-gap margin), and pulling edges back in where they stick out past
+//! the usable area or into a neighbor. Pure frame-space canvas math — the
+//! compositor glue in `state/fill.rs` supplies the rects and converts the
+//! result back to a client size + map location.
+
+use super::snap::SnapRect;
+
+/// Grow `current` outward inside `bounds` (inset by `gap`), never covering an
+/// obstacle and never crossing the gap margin, then apply frame-space size
+/// constraints. `min_size` / `max_size` use `0.0` as the "unconstrained on this
+/// axis" sentinel. Returns `None` when `current` lies entirely outside the
+/// gap-inset bounds — such a window can't be filled without being moved.
+pub fn fill_rect(
+    current: SnapRect,
+    obstacles: &[SnapRect],
+    bounds: SnapRect,
+    gap: f64,
+    min_size: (f64, f64),
+    max_size: (f64, f64),
+) -> Option<SnapRect> {
+    let b = SnapRect {
+        x_low: bounds.x_low + gap,
+        x_high: bounds.x_high - gap,
+        y_low: bounds.y_low + gap,
+        y_high: bounds.y_high - gap,
+    };
+    if !current.overlaps(&b) {
+        return None;
+    }
+
+    // Viewport clamp: pull any edge sticking out past the usable area back in.
+    let clamped = SnapRect {
+        x_low: current.x_low.max(b.x_low),
+        x_high: current.x_high.min(b.x_high),
+        y_low: current.y_low.max(b.y_low),
+        y_high: current.y_high.min(b.y_high),
+    };
+
+    // Shrink out of partial overlaps: pull the least-travel single edge of each
+    // overlapping obstacle back to a gap. Obstacles that can't be escaped this
+    // way (an obstacle enclosing the window, or min-size blocking every pull)
+    // keep overlapping and are ignored below.
+    let resolved = resolve_overlaps(clamped, obstacles, gap, min_size);
+
+    // An obstacle still overlapping after resolution can't be cleared by growth
+    // (that would need a move), so it's ignored throughout; a resolved obstacle
+    // no longer overlaps and rejoins as a normal blocker.
+    let active: Vec<SnapRect> = obstacles
+        .iter()
+        .copied()
+        .filter(|o| !o.overlaps(&resolved))
+        .collect();
+
+    // Axis order decides which of an L-shaped free region's arms a diagonal
+    // blocker cedes; keep whichever order yields the larger area, ties to
+    // horizontal-first.
+    let hv = grow(resolved, &active, b, gap, true);
+    let vh = grow(resolved, &active, b, gap, false);
+    let target = if area(vh) > area(hv) { vh } else { hv };
+
+    Some(apply_constraints(target, resolved, min_size, max_size))
+}
+
+/// Shrink `c` to escape partial overlaps with `obstacles`. Obstacles are visited
+/// in a fixed sorted order, re-checking overlap before each (an earlier shrink
+/// may already have cleared a later one). For an obstacle the rect still
+/// overlaps, the four single-edge pulls back to a gap are considered; a pull is
+/// eligible when it moves the edge inward while leaving that axis no shorter
+/// than its min-size floor (`0.0` → a 1px floor). The eligible pull with the
+/// least edge travel wins (ties broken in x-high, x-low, y-high, y-low order).
+/// When no pull is eligible — an obstacle enclosing the rect, or min-size
+/// blocking every escape — the overlap is left in place. Shrinks only remove
+/// overlaps and never create them, so this single pass terminates.
+fn resolve_overlaps(
+    mut c: SnapRect,
+    obstacles: &[SnapRect],
+    gap: f64,
+    min_size: (f64, f64),
+) -> SnapRect {
+    let min_w = if min_size.0 > 0.0 { min_size.0 } else { 1.0 };
+    let min_h = if min_size.1 > 0.0 { min_size.1 } else { 1.0 };
+
+    let mut order: Vec<&SnapRect> = obstacles.iter().collect();
+    order.sort_by(|a, b| {
+        a.x_low
+            .total_cmp(&b.x_low)
+            .then_with(|| a.y_low.total_cmp(&b.y_low))
+            .then_with(|| a.x_high.total_cmp(&b.x_high))
+            .then_with(|| a.y_high.total_cmp(&b.y_high))
+    });
+
+    for o in order {
+        if !c.overlaps(o) {
+            continue;
+        }
+        // (resulting rect, remaining extent on the pulled axis, edge travel,
+        // min extent for that axis) in tie-break order.
+        let pulls = [
+            (
+                SnapRect {
+                    x_high: o.x_low - gap,
+                    ..c
+                },
+                (o.x_low - gap) - c.x_low,
+                c.x_high - (o.x_low - gap),
+                min_w,
+            ),
+            (
+                SnapRect {
+                    x_low: o.x_high + gap,
+                    ..c
+                },
+                c.x_high - (o.x_high + gap),
+                (o.x_high + gap) - c.x_low,
+                min_w,
+            ),
+            (
+                SnapRect {
+                    y_high: o.y_low - gap,
+                    ..c
+                },
+                (o.y_low - gap) - c.y_low,
+                c.y_high - (o.y_low - gap),
+                min_h,
+            ),
+            (
+                SnapRect {
+                    y_low: o.y_high + gap,
+                    ..c
+                },
+                c.y_high - (o.y_high + gap),
+                (o.y_high + gap) - c.y_low,
+                min_h,
+            ),
+        ];
+        let best = pulls
+            .into_iter()
+            .filter(|&(_, extent, travel, min)| travel > 0.0 && extent >= min)
+            .min_by(|a, b| a.2.total_cmp(&b.2));
+        if let Some((resolved, ..)) = best {
+            c = resolved;
+        }
+    }
+    c
+}
+
+fn area(r: SnapRect) -> f64 {
+    (r.x_high - r.x_low) * (r.y_high - r.y_low)
+}
+
+fn grow(
+    clamped: SnapRect,
+    obstacles: &[SnapRect],
+    bounds: SnapRect,
+    gap: f64,
+    horizontal_first: bool,
+) -> SnapRect {
+    let mut r = clamped;
+    if horizontal_first {
+        grow_horizontal(&mut r, obstacles, bounds, gap);
+        grow_vertical(&mut r, obstacles, bounds, gap);
+    } else {
+        grow_vertical(&mut r, obstacles, bounds, gap);
+        grow_horizontal(&mut r, obstacles, bounds, gap);
+    }
+    r
+}
+
+fn grow_horizontal(r: &mut SnapRect, obstacles: &[SnapRect], bounds: SnapRect, gap: f64) {
+    let mut max_left = f64::NEG_INFINITY;
+    let mut min_right = f64::INFINITY;
+    for o in obstacles {
+        // Blocks only if it overlaps the current perpendicular (vertical)
+        // extent within the gap margin — a diagonal neighbor inside `gap`
+        // still blocks. Each side undoes the gap the way the rect edge was
+        // derived (low = `o + gap`, high = `o - gap`), so an edge already
+        // snapped a gap from `o` compares exactly and re-fills identically.
+        if o.y_high + gap <= r.y_low || o.y_low - gap >= r.y_high {
+            continue;
+        }
+        if o.x_high <= r.x_low {
+            max_left = max_left.max(o.x_high + gap);
+        } else if o.x_low >= r.x_high {
+            min_right = min_right.min(o.x_low - gap);
+        } else {
+            // A straddler inside the perpendicular gap ring fits neither
+            // partition. Growing past a side it already protrudes beyond would
+            // open a fresh sub-gap overhang, so freeze that side by feeding the
+            // current edge into the guard below — which then holds it in place.
+            if o.x_high > r.x_high {
+                min_right = min_right.min(r.x_high);
+            }
+            if o.x_low < r.x_low {
+                max_left = max_left.max(r.x_low);
+            }
+        }
+    }
+    // The outer min/max guard the neighbor-closer-than-gap case: growth only
+    // ever moves an edge outward, never inward.
+    r.x_low = r.x_low.min(max_left.max(bounds.x_low));
+    r.x_high = r.x_high.max(min_right.min(bounds.x_high));
+}
+
+fn grow_vertical(r: &mut SnapRect, obstacles: &[SnapRect], bounds: SnapRect, gap: f64) {
+    let mut max_top = f64::NEG_INFINITY;
+    let mut min_bottom = f64::INFINITY;
+    for o in obstacles {
+        if o.x_high + gap <= r.x_low || o.x_low - gap >= r.x_high {
+            continue;
+        }
+        if o.y_high <= r.y_low {
+            max_top = max_top.max(o.y_high + gap);
+        } else if o.y_low >= r.y_high {
+            min_bottom = min_bottom.min(o.y_low - gap);
+        } else {
+            // Symmetric straddler freeze (see grow_horizontal).
+            if o.y_high > r.y_high {
+                min_bottom = min_bottom.min(r.y_high);
+            }
+            if o.y_low < r.y_low {
+                max_top = max_top.max(r.y_low);
+            }
+        }
+    }
+    r.y_low = r.y_low.min(max_top.max(bounds.y_low));
+    r.y_high = r.y_high.max(min_bottom.min(bounds.y_high));
+}
+
+fn apply_constraints(
+    target: SnapRect,
+    resolved: SnapRect,
+    min_size: (f64, f64),
+    max_size: (f64, f64),
+) -> SnapRect {
+    let target_w = target.x_high - target.x_low;
+    let target_h = target.y_high - target.y_low;
+    let w = clamp_axis(target_w, min_size.0, max_size.0);
+    let h = clamp_axis(target_h, min_size.1, max_size.1);
+    let x_low = anchor_low(target.x_low, target.x_high, resolved.x_low, w, target_w);
+    let y_low = anchor_low(target.y_low, target.y_high, resolved.y_low, h, target_h);
+    SnapRect {
+        x_low,
+        x_high: x_low + w,
+        y_low,
+        y_high: y_low + h,
+    }
+}
+
+/// `0.0` bounds are unconstrained; the low bound floors at 1px. Ordered so a
+/// contradictory client (min > max) yields the max rather than panicking.
+fn clamp_axis(len: f64, min: f64, max: f64) -> f64 {
+    let lo = if min > 0.0 { min } else { 1.0 };
+    let hi = if max > 0.0 { max } else { f64::INFINITY };
+    len.max(lo).min(hi)
+}
+
+/// When a max-size cap shrinks the axis below the grown length, keep the
+/// overlap-resolved low edge (give up growth on the high side first); when a
+/// min-size floor forces it larger, keep the target's low edge and let it
+/// overflow on the high side.
+fn anchor_low(
+    target_low: f64,
+    target_high: f64,
+    resolved_low: f64,
+    len: f64,
+    target_len: f64,
+) -> f64 {
+    if len < target_len {
+        resolved_low.min(target_high - len).max(target_low)
+    } else {
+        target_low
+    }
+}
+
+#[cfg(test)]
+mod tests {
+    use super::*;
+    use proptest::prelude::*;
+
+    fn rect(x_low: f64, y_low: f64, w: f64, h: f64) -> SnapRect {
+        SnapRect {
+            x_low,
+            x_high: x_low + w,
+            y_low,
+            y_high: y_low + h,
+        }
+    }
+
+    const UNCONSTRAINED: (f64, f64) = (0.0, 0.0);
+
+    /// Bounds a comfortable 1000×1000 room with a 10px gap → free region
+    /// [10,990]².
+    fn room() -> SnapRect {
+        rect(0.0, 0.0, 1000.0, 1000.0)
+    }
+
+    fn approx(a: f64, b: f64) -> bool {
+        (a - b).abs() < 1e-6
+    }
+
+    fn rect_approx(a: SnapRect, b: SnapRect) -> bool {
+        approx(a.x_low, b.x_low)
+            && approx(a.x_high, b.x_high)
+            && approx(a.y_low, b.y_low)
+            && approx(a.y_high, b.y_high)
+    }
+
+    #[test]
+    fn no_obstacles_grows_to_inset_bounds() {
+        let cur = rect(400.0, 400.0, 100.0, 100.0);
+        let out = fill_rect(cur, &[], room(), 10.0, UNCONSTRAINED, UNCONSTRAINED).unwrap();
+        assert!(rect_approx(out, rect(10.0, 10.0, 980.0, 980.0)));
+    }
+
+    #[test]
+    fn right_obstacle_stops_at_gap() {
+        let cur = rect(100.0, 400.0, 100.0, 100.0);
+        let neighbor = rect(600.0, 300.0, 200.0, 300.0);
+        let out = fill_rect(cur, &[neighbor], room(), 10.0, UNCONSTRAINED, UNCONSTRAINED).unwrap();
+        // Right edge stops a gap short of the neighbor's left edge (590),
+        // other edges reach the inset bounds.
+        assert!(approx(out.x_high, 590.0), "x_high = {}", out.x_high);
+        assert!(approx(out.x_low, 10.0));
+        assert!(approx(out.y_low, 10.0));
+        assert!(approx(out.y_high, 990.0));
+    }
+
+    #[test]
+    fn neighbor_closer_than_gap_edge_never_pulls_inward() {
+        // Neighbour's left edge (505) is less than a gap from the window's
+        // right edge (500): the right edge must stay, not retreat.
+        let cur = rect(100.0, 400.0, 400.0, 100.0);
+        let neighbor = rect(505.0, 300.0, 200.0, 300.0);
+        let out = fill_rect(cur, &[neighbor], room(), 10.0, UNCONSTRAINED, UNCONSTRAINED).unwrap();
+        assert!(approx(out.x_high, 500.0), "x_high = {}", out.x_high);
+    }
+
+    #[test]
+    fn fully_containing_obstacle_is_ignored() {
+        let cur = rect(400.0, 400.0, 200.0, 200.0);
+        // Encloses the window on every side — no single-edge pull escapes it, so
+        // fill ignores it and grows over it to the full inset bounds (the old
+        // "obstacle overlapping the window" behavior).
+        let enclosing = rect(300.0, 300.0, 400.0, 400.0);
+        let out = fill_rect(
+            cur,
+            &[enclosing],
+            room(),
+            10.0,
+            UNCONSTRAINED,
+            UNCONSTRAINED,
+        )
+        .unwrap();
+        assert!(rect_approx(out, rect(10.0, 10.0, 980.0, 980.0)));
+    }
+
+    #[test]
+    fn partial_overlap_resolves_to_gap_then_grows() {
+        // Window pokes into a neighbor on its right; the right edge pulls back to
+        // exactly a gap short of the neighbor, and the other three edges grow to
+        // the inset bounds.
+        let cur = rect(400.0, 400.0, 200.0, 100.0); // [400,600]×[400,500]
+        let neighbor = rect(550.0, 300.0, 200.0, 300.0); // [550,750]×[300,600]
+        let out = fill_rect(cur, &[neighbor], room(), 10.0, UNCONSTRAINED, UNCONSTRAINED).unwrap();
+        assert!(approx(out.x_high, 540.0), "x_high = {}", out.x_high); // 550 − gap
+        assert!(approx(out.x_low, 10.0), "x_low = {}", out.x_low);
+        assert!(approx(out.y_low, 10.0));
+        assert!(approx(out.y_high, 990.0));
+    }
+
+    #[test]
+    fn left_partial_overlap_resolves_to_gap() {
+        // Neighbor on the left: the left edge pulls out to a gap past the
+        // neighbor's right edge, then the free sides grow to bounds.
+        let cur = rect(400.0, 400.0, 200.0, 200.0); // [400,600]²
+        let neighbor = rect(250.0, 300.0, 200.0, 400.0); // [250,450]×[300,700]
+        let out = fill_rect(cur, &[neighbor], room(), 10.0, UNCONSTRAINED, UNCONSTRAINED).unwrap();
+        assert!(approx(out.x_low, 460.0), "x_low = {}", out.x_low); // 450 + gap
+        assert!(approx(out.x_high, 990.0));
+        assert!(approx(out.y_low, 10.0));
+        assert!(approx(out.y_high, 990.0));
+    }
+
+    #[test]
+    fn minimal_travel_edge_is_chosen() {
+        // A corner overlap where the horizontal escape travels less (30) than the
+        // vertical one (60): the x-high edge is pulled, and vertical growth is
+        // left free (reaches the bounds rather than the y-escape at 540).
+        let cur = rect(400.0, 400.0, 200.0, 200.0); // [400,600]²
+        let neighbor = rect(580.0, 550.0, 200.0, 200.0); // [580,780]×[550,750]
+        let out = fill_rect(cur, &[neighbor], room(), 10.0, UNCONSTRAINED, UNCONSTRAINED).unwrap();
+        assert!(approx(out.x_high, 570.0), "x_high = {}", out.x_high); // 580 − gap
+        assert!(approx(out.y_high, 990.0), "y_high = {}", out.y_high);
+    }
+
+    #[test]
+    fn tie_break_prefers_x_high() {
+        // Symmetric corner overlap: the x-high and y-high pulls travel equally
+        // (20 each). The deterministic tie-break takes x-high first, so vertical
+        // growth is untouched.
+        let cur = rect(400.0, 400.0, 200.0, 200.0); // [400,600]²
+        let neighbor = rect(590.0, 590.0, 200.0, 200.0); // [590,790]²
+        let out = fill_rect(cur, &[neighbor], room(), 10.0, UNCONSTRAINED, UNCONSTRAINED).unwrap();
+        assert!(approx(out.x_high, 580.0), "x_high = {}", out.x_high); // 590 − gap
+        assert!(approx(out.y_high, 990.0), "y_high = {}", out.y_high);
+    }
+
+    #[test]
+    fn min_size_can_block_every_resolving_pull() {
+        // Neighbor overlaps the right; the only escaping pull (x-high) would leave
+        // a 140px-wide rect. Unconstrained that resolves; a 200px min width blocks
+        // it, so the obstacle is ignored and growth covers it instead.
+        let cur = rect(400.0, 400.0, 200.0, 200.0); // [400,600]²
+        let neighbor = rect(550.0, 300.0, 200.0, 400.0); // [550,750]×[300,700]
+
+        let free = fill_rect(cur, &[neighbor], room(), 10.0, UNCONSTRAINED, UNCONSTRAINED).unwrap();
+        assert!(approx(free.x_high, 540.0), "x_high = {}", free.x_high);
+
+        let blocked =
+            fill_rect(cur, &[neighbor], room(), 10.0, (200.0, 0.0), UNCONSTRAINED).unwrap();
+        assert!(rect_approx(blocked, rect(10.0, 10.0, 980.0, 980.0)));
+    }
+
+    #[test]
+    fn two_overlaps_resolved_in_one_pass() {
+        // One neighbor on the right, one below, each overlapping a corner of the
+        // window; both edges pull back to a gap in a single pass and the window
+        // grows into the free top-left.
+        let cur = rect(400.0, 400.0, 200.0, 200.0); // [400,600]²
+        let right = rect(550.0, 300.0, 200.0, 600.0); // [550,750]×[300,900]
+        let below = rect(300.0, 550.0, 600.0, 200.0); // [300,900]×[550,750]
+        let out = fill_rect(
+            cur,
+            &[right, below],
+            room(),
+            10.0,
+            UNCONSTRAINED,
+            UNCONSTRAINED,
+        )
+        .unwrap();
+        assert!(approx(out.x_high, 540.0), "x_high = {}", out.x_high); // 550 − gap
+        assert!(approx(out.y_high, 540.0), "y_high = {}", out.y_high); // 550 − gap
+        assert!(approx(out.x_low, 10.0));
+        assert!(approx(out.y_low, 10.0));
+    }
+
+    #[test]
+    fn shrink_rechecks_incidentally_resolved_obstacle() {
+        // Pulling the x-high edge to clear the first (nearer) neighbor also lifts
+        // the window clear of the second one, whose perpendicular pull would
+        // otherwise have spuriously shrunk the window vertically. The re-check
+        // skips it, so vertical growth reaches the bounds.
+        let cur = rect(400.0, 400.0, 200.0, 200.0); // [400,600]²
+        let first = rect(500.0, 300.0, 300.0, 400.0); // [500,800]×[300,700]
+        let second = rect(520.0, 550.0, 300.0, 300.0); // [520,820]×[550,850]
+        let out = fill_rect(
+            cur,
+            &[first, second],
+            room(),
+            10.0,
+            UNCONSTRAINED,
+            UNCONSTRAINED,
+        )
+        .unwrap();
+        assert!(approx(out.x_high, 490.0), "x_high = {}", out.x_high); // 500 − gap
+        assert!(approx(out.y_high, 990.0), "y_high = {}", out.y_high);
+    }
+
+    #[test]
+    fn neighbor_below_within_gap_does_not_block_horizontal_growth() {
+        // A neighbor sits a few px below the window (inside the gap margin) with
+        // the same x-range: it doesn't overlap, so it isn't shrunk against, and it
+        // fits neither the left-of nor right-of partition for horizontal growth —
+        // horizontal growth must proceed to the bounds. Downward growth stays put
+        // (the outer max-guard keeps the too-close edge from retreating).
+        let cur = rect(100.0, 100.0, 100.0, 100.0); // [100,200]²
+        let neighbor = rect(100.0, 205.0, 100.0, 200.0); // [100,200]×[205,405]
+        let out = fill_rect(cur, &[neighbor], room(), 10.0, UNCONSTRAINED, UNCONSTRAINED).unwrap();
+        assert!(approx(out.x_low, 10.0), "x_low = {}", out.x_low);
+        assert!(approx(out.x_high, 990.0), "x_high = {}", out.x_high);
+        assert!(approx(out.y_low, 10.0), "y_low = {}", out.y_low);
+        assert!(approx(out.y_high, 200.0), "y_high = {}", out.y_high); // not pulled to 195
+    }
+
+    #[test]
+    fn straddler_protruding_right_freezes_only_right_growth() {
+        // A neighbor a few px below (inside the gap margin) straddles the
+        // window's x-range and protrudes past its right edge. Growing right
+        // would open a sub-gap overhang across x∈[200,400]; only the right edge
+        // freezes, the left still grows to the bounds.
+        let cur = rect(100.0, 100.0, 100.0, 100.0); // [100,200]²
+        let neighbor = rect(150.0, 205.0, 250.0, 200.0); // [150,400]×[205,405]
+        let out = fill_rect(cur, &[neighbor], room(), 10.0, UNCONSTRAINED, UNCONSTRAINED).unwrap();
+        assert!(approx(out.x_high, 200.0), "x_high = {}", out.x_high);
+        assert!(approx(out.x_low, 10.0), "x_low = {}", out.x_low);
+    }
+
+    #[test]
+    fn straddler_protruding_left_freezes_only_left_growth() {
+        // Mirror image: the straddler protrudes past the window's left edge, so
+        // only the left edge freezes and the right grows to the bounds.
+        let cur = rect(100.0, 100.0, 100.0, 100.0); // [100,200]²
+        let neighbor = rect(50.0, 205.0, 100.0, 200.0); // [50,150]×[205,405]
+        let out = fill_rect(cur, &[neighbor], room(), 10.0, UNCONSTRAINED, UNCONSTRAINED).unwrap();
+        assert!(approx(out.x_low, 100.0), "x_low = {}", out.x_low);
+        assert!(approx(out.x_high, 990.0), "x_high = {}", out.x_high);
+    }
+
+    #[test]
+    fn straddler_exactly_gap_away_does_not_freeze() {
+        // The same protruding straddler, but parked exactly a gap below (not
+        // sub-gap): it falls outside the perpendicular gap ring, so it is never
+        // considered and horizontal growth reaches the bounds unfrozen.
+        let cur = rect(100.0, 100.0, 100.0, 100.0); // [100,200]²
+        let neighbor = rect(150.0, 210.0, 250.0, 200.0); // y_low = 200 + gap
+        let out = fill_rect(cur, &[neighbor], room(), 10.0, UNCONSTRAINED, UNCONSTRAINED).unwrap();
+        assert!(approx(out.x_high, 990.0), "x_high = {}", out.x_high);
+        assert!(approx(out.x_low, 10.0), "x_low = {}", out.x_low);
+    }
+
+    #[test]
+    fn diagonal_within_gap_margin_blocks() {
+        // Neighbour sits below-right, not vertically overlapping the window yet,
+        // but within the gap margin perpendicular to horizontal growth — it must
+        // still cap the right edge.
+        let cur = rect(100.0, 100.0, 100.0, 100.0);
+        // window y: [100,200]. neighbor y_low = 205 → within gap (10) of 200.
+        let neighbor = rect(600.0, 205.0, 200.0, 200.0);
+        let out = fill_rect(cur, &[neighbor], room(), 10.0, UNCONSTRAINED, UNCONSTRAINED).unwrap();
+        assert!(approx(out.x_high, 590.0), "x_high = {}", out.x_high);
+    }
+
+    #[test]
+    fn l_shaped_free_space_picks_larger_area_axis_order() {
+        // One obstacle blocks horizontal growth low-down; growing vertical
+        // first frees the full-height narrow arm, horizontal first frees the
+        // full-width short arm. Pick the larger.
+        let cur = rect(100.0, 100.0, 100.0, 100.0);
+        // Blocks the right for y in [300, 990]; leaves the top-right open.
+        let obstacle = rect(400.0, 300.0, 590.0, 690.0);
+        let out = fill_rect(cur, &[obstacle], room(), 10.0, UNCONSTRAINED, UNCONSTRAINED).unwrap();
+        // Neither candidate may overlap the obstacle.
+        assert!(!out.overlaps(&obstacle));
+        // Vertical-first arm: full height [10,990], width capped at 390 (gap
+        // before obstacle) = area 980*380. Horizontal-first: full width [10,990]
+        // but height capped at 290 = 980*280. Vertical-first is larger.
+        assert!(
+            approx(out.y_low, 10.0) && approx(out.y_high, 990.0),
+            "{out:?}"
+        );
+        assert!(approx(out.x_high, 390.0), "x_high = {}", out.x_high);
+    }
+
+    #[test]
+    fn partially_out_window_shrinks_back_to_bounds() {
+        // Window overhangs the left and top of the usable area; fill pulls those
+        // edges in to the inset bounds while growing the others out.
+        let cur = rect(-200.0, -200.0, 400.0, 400.0);
+        let out = fill_rect(cur, &[], room(), 10.0, UNCONSTRAINED, UNCONSTRAINED).unwrap();
+        assert!(rect_approx(out, rect(10.0, 10.0, 980.0, 980.0)));
+    }
+
+    #[test]
+    fn fully_outside_returns_none() {
+        let cur = rect(1200.0, 1200.0, 100.0, 100.0);
+        assert!(fill_rect(cur, &[], room(), 10.0, UNCONSTRAINED, UNCONSTRAINED).is_none());
+    }
+
+    #[test]
+    fn min_size_floor_anchors_low_edge() {
+        // A min width larger than the free region: keep the target's low edge
+        // and overflow on the high side.
+        let cur = rect(400.0, 400.0, 100.0, 100.0);
+        let out = fill_rect(cur, &[], room(), 10.0, (2000.0, 0.0), UNCONSTRAINED).unwrap();
+        assert!(approx(out.x_low, 10.0), "x_low = {}", out.x_low);
+        assert!(approx(out.x_high - out.x_low, 2000.0));
+    }
+
+    #[test]
+    fn max_size_cap_keeps_left_edge() {
+        // Window sits at the left; a max width smaller than the free region caps
+        // growth on the right, keeping the viewport-clamped left edge.
+        let cur = rect(10.0, 400.0, 100.0, 100.0);
+        let out = fill_rect(cur, &[], room(), 10.0, UNCONSTRAINED, (300.0, 0.0)).unwrap();
+        assert!(approx(out.x_low, 10.0), "x_low = {}", out.x_low);
+        assert!(approx(out.x_high, 310.0), "x_high = {}", out.x_high);
+    }
+
+    #[test]
+    fn respects_gap_against_bounds() {
+        let cur = rect(400.0, 400.0, 100.0, 100.0);
+        let out = fill_rect(cur, &[], room(), 25.0, UNCONSTRAINED, UNCONSTRAINED).unwrap();
+        assert!(approx(out.x_low, 25.0) && approx(out.x_high, 975.0));
+        assert!(approx(out.y_low, 25.0) && approx(out.y_high, 975.0));
+    }
+
+    /// A rect strategy inside a generous canvas, sizes kept positive.
+    fn any_rect() -> impl Strategy<Value = SnapRect> {
+        (
+            -500.0..500.0f64,
+            -500.0..500.0f64,
+            20.0..600.0f64,
+            20.0..600.0f64,
+        )
+            .prop_map(|(x, y, w, h)| rect(x, y, w, h))
+    }
+
+    fn gap_inset(bounds: SnapRect, gap: f64) -> SnapRect {
+        SnapRect {
+            x_low: bounds.x_low + gap,
+            x_high: bounds.x_high - gap,
+            y_low: bounds.y_low + gap,
+            y_high: bounds.y_high - gap,
+        }
+    }
+
+    /// `inner` is inside `outer` up to a float tolerance.
+    fn within(inner: SnapRect, outer: SnapRect) -> bool {
+        const EPS: f64 = 1e-6;
+        inner.x_low >= outer.x_low - EPS
+            && inner.x_high <= outer.x_high + EPS
+            && inner.y_low >= outer.y_low - EPS
+            && inner.y_high <= outer.y_high + EPS
+    }
+
+    /// `inner` contains `outer` up to a float tolerance.
+    fn contains(inner: SnapRect, outer: SnapRect) -> bool {
+        const EPS: f64 = 1e-6;
+        inner.x_low <= outer.x_low + EPS
+            && inner.x_high >= outer.x_high - EPS
+            && inner.y_low <= outer.y_low + EPS
+            && inner.y_high >= outer.y_high - EPS
+    }
+
+    proptest! {
+        #[test]
+        fn result_within_bounds_only_keeps_unresolvable_overlaps(
+            current in any_rect(),
+            obstacles in prop::collection::vec(any_rect(), 0..6),
+            gap in 0.0..30.0f64,
+        ) {
+            let bounds = rect(-800.0, -800.0, 1600.0, 1600.0);
+            if let Some(out) = fill_rect(current, &obstacles, bounds, gap, UNCONSTRAINED, UNCONSTRAINED) {
+                let b = gap_inset(bounds, gap);
+                // Unconstrained, so the result fits the gap-inset bounds.
+                prop_assert!(within(out, b), "result {out:?} escaped inset bounds {b:?}");
+
+                // Growth starts from the shrunk (resolved) rect, so the result
+                // contains that — not necessarily the raw viewport clamp.
+                let clamped = SnapRect {
+                    x_low: current.x_low.max(b.x_low),
+                    x_high: current.x_high.min(b.x_high),
+                    y_low: current.y_low.max(b.y_low),
+                    y_high: current.y_high.min(b.y_high),
+                };
+                let resolved = resolve_overlaps(clamped, &obstacles, gap, UNCONSTRAINED);
+                prop_assert!(contains(out, resolved), "result {out:?} lost resolved rect {resolved:?}");
+
+                // The result may only overlap obstacles the shrink phase could
+                // not escape (they still overlap the resolved rect); every
+                // resolvable obstacle is cleared.
+                for o in &obstacles {
+                    if !resolved.overlaps(o) {
+                        prop_assert!(!out.overlaps(o), "result {out:?} overlaps resolvable obstacle {o:?}");
+                    }
+                }
+            }
+        }
+
+        #[test]
+        fn idempotent_from_steady_state(
+            current in any_rect(),
+            obstacles in prop::collection::vec(any_rect(), 0..6),
+            gap in 0.0..30.0f64,
+        ) {
+            let bounds = rect(-800.0, -800.0, 1600.0, 1600.0);
+            if let Some(first) = fill_rect(current, &obstacles, bounds, gap, UNCONSTRAINED, UNCONSTRAINED) {
+                // Once a fill no longer overlaps any obstacle — the steady state a
+                // fill reaches unless it had to grow over an unresolvable
+                // enclosing obstacle — re-filling reproduces it exactly.
+                if obstacles.iter().all(|o| !first.overlaps(o)) {
+                    let second = fill_rect(first, &obstacles, bounds, gap, UNCONSTRAINED, UNCONSTRAINED)
+                        .expect("filled rect still intersects bounds");
+                    prop_assert!(rect_approx(first, second), "not idempotent: {first:?} -> {second:?}");
+                }
+            }
+        }
+    }
+}
diff --git a/src/layout/mod.rs b/src/layout/mod.rs
index c6b755e..4212103 100644
--- a/src/layout/mod.rs
+++ b/src/layout/mod.rs
@@ -1,3 +1,4 @@
 pub mod auto_placement;
 pub mod cluster;
+pub mod fill;
 pub mod snap;
diff --git a/src/render/closing.rs b/src/render/closing.rs
new file mode 100644
index 0000000..e3dabba
--- /dev/null
+++ b/src/render/closing.rs
@@ -0,0 +1,160 @@
+use smithay::backend::allocator::Fourcc;
+use smithay::backend::renderer::element::texture::{TextureBuffer, TextureRenderElement};
+use smithay::backend::renderer::element::{Element, Kind, RenderElement};
+use smithay::backend::renderer::gles::{GlesError, GlesRenderer, GlesTexture};
+use smithay::backend::renderer::{Bind as _, Color32F, Frame as _, Renderer as _};
+use smithay::utils::user_data::UserDataMap;
+use smithay::utils::{Logical, Physical, Point, Rectangle, Scale, Size, Transform};
+
+use super::{OutputRenderElements, WindowTransformElement};
+
+const CLOSE_SCALE: f64 = 0.8;
+const DONE_EPSILON: f64 = 0.001;
+
+/// Short-lived GPU snapshot used after the real window has already left the
+/// stage. One texture and one affine render element keep closing animations
+/// independent of client teardown and cheap to draw.
+#[derive(Debug)]
+pub(crate) struct ClosingSnapshot {
+    buffer: TextureBuffer<GlesTexture>,
+    output: String,
+    geometry: Rectangle<i32, Physical>,
+    logical_size: Size<i32, Logical>,
+    camera: Point<f64, Logical>,
+    zoom: f64,
+    pinned: bool,
+    progress: f64,
+}
+
+impl ClosingSnapshot {
+    pub fn tick(&mut self, frame_factor: f64) {
+        self.progress += (1.0 - self.progress) * frame_factor;
+    }
+
+    pub fn is_done(&self) -> bool {
+        1.0 - self.progress <= DONE_EPSILON
+    }
+
+    fn render_element(
+        &self,
+        camera: Point<f64, Logical>,
+        zoom: f64,
+        output_scale: f64,
+    ) -> OutputRenderElements {
+        let alpha = (1.0 - self.progress).clamp(0.0, 1.0) as f32;
+        let texture = TextureRenderElement::from_texture_buffer(
+            self.geometry.loc.to_f64(),
+            &self.buffer,
+            Some(alpha),
+            None,
+            Some(self.logical_size),
+            Kind::Unspecified,
+        );
+        let close_scale = 1.0 - (1.0 - CLOSE_SCALE) * self.progress;
+        let zoom_ratio = if self.pinned { 1.0 } else { zoom / self.zoom };
+        let camera_offset: Point<f64, Physical> = if self.pinned {
+            Point::default()
+        } else {
+            Point::from((
+                (self.camera.x - camera.x) * zoom * output_scale,
+                (self.camera.y - camera.y) * zoom * output_scale,
+            ))
+        };
+        let captured_center =
+            self.geometry.loc.to_f64() + self.geometry.size.to_f64().to_point().downscale(2.0);
+        let offset = camera_offset
+            + captured_center
+                .upscale(zoom_ratio)
+                .upscale(1.0 - close_scale);
+        OutputRenderElements::ClosingWindow(WindowTransformElement::new(
+            texture,
+            Point::default(),
+            offset,
+            Scale::from(zoom_ratio * close_scale),
+        ))
+    }
+}
+
+pub(crate) fn capture(
+    renderer: &mut GlesRenderer,
+    output: &str,
+    output_scale: Scale<f64>,
+    camera: Point<f64, Logical>,
+    zoom: f64,
+    pinned: bool,
+    elements: &[OutputRenderElements],
+) -> Result<Option<ClosingSnapshot>, GlesError> {
+    let Some(geometry) = elements
+        .iter()
+        .map(|element| element.geometry(output_scale))
+        .reduce(|a, b| a.merge(b))
+        .filter(|geometry| geometry.size.w > 0 && geometry.size.h > 0)
+    else {
+        return Ok(None);
+    };
+
+    let buffer_size = geometry.size.to_logical(1).to_buffer(1, Transform::Normal);
+    let mut texture =
+        <GlesRenderer as smithay::backend::renderer::Offscreen<GlesTexture>>::create_buffer(
+            renderer,
+            Fourcc::Abgr8888,
+            buffer_size,
+        )?;
+    {
+        let mut target = renderer.bind(&mut texture)?;
+        let mut frame = renderer.render(&mut target, geometry.size, Transform::Normal)?;
+        frame.clear(
+            Color32F::TRANSPARENT,
+            &[Rectangle::from_size(geometry.size)],
+        )?;
+
+        // OutputRenderElements are front-to-back. An offscreen framebuffer is
+        // painter's algorithm, so draw them in reverse.
+        for element in elements.iter().rev() {
+            let src = element.src();
+            let mut dst = element.geometry(output_scale);
+            dst.loc -= geometry.loc;
+            let Some(mut damage) = Rectangle::from_size(geometry.size).intersection(dst) else {
+                continue;
+            };
+            damage.loc -= dst.loc;
+            let cache = UserDataMap::new();
+            if element.is_framebuffer_effect() {
+                element.capture_framebuffer(&mut frame, src, dst, &cache)?;
+            }
+            element.draw(&mut frame, src, dst, &[damage], &[], Some(&cache))?;
+        }
+        let _sync = frame.finish()?;
+    }
+
+    let buffer = TextureBuffer::from_texture(renderer, texture, 1, Transform::Normal, None);
+    let logical_size = geometry
+        .size
+        .to_f64()
+        .to_logical(output_scale)
+        .to_i32_round();
+    Ok(Some(ClosingSnapshot {
+        buffer,
+        output: output.to_owned(),
+        geometry,
+        logical_size,
+        camera,
+        zoom,
+        pinned,
+        progress: 0.0,
+    }))
+}
+
+pub(crate) fn render_for_output(
+    snapshots: &[ClosingSnapshot],
+    output: &str,
+    camera: Point<f64, Logical>,
+    zoom: f64,
+    output_scale: f64,
+) -> Vec<OutputRenderElements> {
+    snapshots
+        .iter()
+        .filter(|snapshot| snapshot.output == output)
+        .map(|snapshot| snapshot.render_element(camera, zoom, output_scale))
+        .collect()
+}
diff --git a/src/render/elements.rs b/src/render/elements.rs
index 6bde23b..fdab7df 100644
--- a/src/render/elements.rs
+++ b/src/render/elements.rs
@@ -221,6 +221,110 @@ pub struct PixelSnapRescaleElement<E> {
     scale: Scale<f64>,
 }
 
+/// Lightweight per-window affine transform used by lifecycle and geometry
+/// animations. It wraps the already zoomed element, so the canvas transform
+/// and the window-local transform remain independent.
+#[derive(Debug)]
+pub struct WindowTransformElement<E> {
+    element: E,
+    origin: Point<f64, Physical>,
+    offset: Point<f64, Physical>,
+    scale: Scale<f64>,
+}
+
+impl<E> WindowTransformElement<E> {
+    pub fn new(
+        element: E,
+        origin: Point<f64, Physical>,
+        offset: Point<f64, Physical>,
+        scale: Scale<f64>,
+    ) -> Self {
+        Self {
+            element,
+            origin,
+            offset,
+            scale,
+        }
+    }
+
+    fn transform_rect(&self, rect: Rectangle<i32, Physical>) -> Rectangle<i32, Physical> {
+        let x0 = self.origin.x + (rect.loc.x as f64 - self.origin.x) * self.scale.x + self.offset.x;
+        let y0 = self.origin.y + (rect.loc.y as f64 - self.origin.y) * self.scale.y + self.offset.y;
+        let x1 = self.origin.x
+            + ((rect.loc.x + rect.size.w) as f64 - self.origin.x) * self.scale.x
+            + self.offset.x;
+        let y1 = self.origin.y
+            + ((rect.loc.y + rect.size.h) as f64 - self.origin.y) * self.scale.y
+            + self.offset.y;
+        Rectangle::new(
+            Point::from((x0.round() as i32, y0.round() as i32)),
+            Size::from((
+                (x1.round() as i32 - x0.round() as i32).max(0),
+                (y1.round() as i32 - y0.round() as i32).max(0),
+            )),
+        )
+    }
+}
+
+impl<E: Element> Element for WindowTransformElement<E> {
+    fn id(&self) -> &Id {
+        self.element.id()
+    }
+    fn current_commit(&self) -> CommitCounter {
+        self.element.current_commit()
+    }
+    fn src(&self) -> Rectangle<f64, smithay::utils::Buffer> {
+        self.element.src()
+    }
+    fn transform(&self) -> Transform {
+        self.element.transform()
+    }
+    fn geometry(&self, scale: Scale<f64>) -> Rectangle<i32, Physical> {
+        self.transform_rect(self.element.geometry(scale))
+    }
+    fn damage_since(
+        &self,
+        scale: Scale<f64>,
+        commit: Option<CommitCounter>,
+    ) -> DamageSet<i32, Physical> {
+        self.element
+            .damage_since(scale, commit)
+            .into_iter()
+            .map(|rect| rect.to_f64().upscale(self.scale).to_i32_up())
+            .collect()
+    }
+    fn opaque_regions(&self, _scale: Scale<f64>) -> OpaqueRegions<i32, Physical> {
+        // Animated alpha and fractional transforms make conservative opaque
+        // tracking more valuable than a small occlusion optimization.
+        OpaqueRegions::default()
+    }
+    fn alpha(&self) -> f32 {
+        self.element.alpha()
+    }
+    fn kind(&self) -> Kind {
+        self.element.kind()
+    }
+}
+
+impl<E: RenderElement<GlesRenderer>> RenderElement<GlesRenderer> for WindowTransformElement<E> {
+    fn draw(
+        &self,
+        frame: &mut GlesFrame<'_, '_>,
+        src: Rectangle<f64, smithay::utils::Buffer>,
+        dst: Rectangle<i32, Physical>,
+        damage: &[Rectangle<i32, Physical>],
+        opaque_regions: &[Rectangle<i32, Physical>],
+        cache: Option<&smithay::utils::user_data::UserDataMap>,
+    ) -> Result<(), GlesError> {
+        self.element
+            .draw(frame, src, dst, damage, opaque_regions, cache)
+    }
+
+    fn underlying_storage(&self, renderer: &mut GlesRenderer) -> Option<UnderlyingStorage<'_>> {
+        self.element.underlying_storage(renderer)
+    }
+}
+
 impl<E: Element> PixelSnapRescaleElement<E> {
     pub fn from_element(
         element: E,
@@ -348,6 +452,11 @@ render_elements! {
     Decoration=PixelSnapRescaleElement<MemoryRenderBufferRenderElement<GlesRenderer>>,
     Window=PixelSnapRescaleElement<WaylandSurfaceRenderElement<GlesRenderer>>,
     CsdWindow=PixelSnapRescaleElement<RoundedCornerElement>,
+    AnimatedDecoration=WindowTransformElement<PixelSnapRescaleElement<MemoryRenderBufferRenderElement<GlesRenderer>>>,
+    AnimatedWindow=WindowTransformElement<PixelSnapRescaleElement<WaylandSurfaceRenderElement<GlesRenderer>>>,
+    AnimatedCsdWindow=WindowTransformElement<PixelSnapRescaleElement<RoundedCornerElement>>,
+    AnimatedChrome=WindowTransformElement<RescaleRenderElement<PixelShaderElement>>,
+    ClosingWindow=WindowTransformElement<TextureRenderElement<GlesTexture>>,
     Layer=WaylandSurfaceRenderElement<GlesRenderer>,
     Cursor=MemoryRenderBufferRenderElement<GlesRenderer>,
     CursorSurface=smithay::backend::renderer::element::Wrap<WaylandSurfaceRenderElement<GlesRenderer>>,
diff --git a/src/render/layers.rs b/src/render/layers.rs
index d43ea3d..ddab4c8 100644
--- a/src/render/layers.rs
+++ b/src/render/layers.rs
@@ -85,6 +85,7 @@ fn push_layer_chrome(
             [r, r, r, r],
             zoom,
             output_scale,
+            None,
         );
     } else {
         push_plain(target, surface_elements);
@@ -108,6 +109,7 @@ fn push_layer_chrome(
             opacity,
             scale,
             zoom,
+            None,
         );
     }
 
@@ -133,6 +135,7 @@ fn push_layer_chrome(
             opacity,
             scale,
             zoom,
+            None,
         );
     }
 
@@ -266,7 +269,7 @@ pub(super) fn build_canvas_layer_elements(
             scale,
             output_scale,
             zoom,
-            |target, elems| super::push_plain_elements(target, elems, zoom),
+            |target, elems| super::push_plain_elements(target, elems, zoom, None),
         );
     }
 
diff --git a/src/render/mod.rs b/src/render/mod.rs
index 6027436..69f8c91 100644
--- a/src/render/mod.rs
+++ b/src/render/mod.rs
@@ -2,6 +2,7 @@ mod background;
 mod blur;
 mod capture;
 mod capture_background;
+mod closing;
 mod cursor;
 mod elements;
 mod error_bar;
@@ -18,9 +19,11 @@ pub use background::{BackgroundElement, init_background, update_background_eleme
 pub(crate) use blur::compile_blur_shaders;
 pub use blur::{BlurCache, SharedBlur};
 pub use capture::{render_capture_frames, render_screencopy, render_toplevel_captures};
+pub(crate) use closing::ClosingSnapshot;
 pub use cursor::build_cursor_elements;
 pub use elements::{
     OutputRenderElements, PixelSnapRescaleElement, RoundedCornerElement, TileShaderElement,
+    WindowTransformElement,
 };
 pub use error_bar::ErrorBarCache;
 pub use lifecycle::{
@@ -39,6 +42,13 @@ use blur::{BlurLayer, BlurRequestData, process_blur_requests};
 use layers::{build_canvas_layer_elements, build_layer_elements};
 use shaders::{push_border_element, push_shadow_element};
 
+#[derive(Clone, Copy)]
+pub(super) struct WindowRenderAnimation {
+    origin: Point<f64, Physical>,
+    offset: Point<f64, Physical>,
+    scale: Scale<f64>,
+}
+
 use smithay::backend::allocator::Fourcc;
 use smithay::backend::renderer::{
     element::{
@@ -109,6 +119,7 @@ fn push_corner_clipped_elements(
     corner_radius: [f32; 4],
     zoom: f64,
     output_scale: f64,
+    animation: Option<WindowRenderAnimation>,
 ) {
     let aa_scale = (output_scale * zoom) as f32;
     // Clamp radii so a tiny window doesn't get corners wider than half its
@@ -122,20 +133,30 @@ fn push_corner_clipped_elements(
         corner_radius[3].clamp(0.0, max_r),
     ];
     for elem in elems {
-        target.push(OutputRenderElements::CsdWindow(
-            PixelSnapRescaleElement::from_element(
-                RoundedCornerElement::new(
+        let elem = PixelSnapRescaleElement::from_element(
+            RoundedCornerElement::new(
+                elem,
+                shader.clone(),
+                geometry,
+                clamped,
+                output_scale,
+                aa_scale,
+            ),
+            Point::<i32, Physical>::from((0, 0)),
+            zoom,
+        );
+        if let Some(animation) = animation {
+            target.push(OutputRenderElements::AnimatedCsdWindow(
+                WindowTransformElement::new(
                     elem,
-                    shader.clone(),
-                    geometry,
-                    clamped,
-                    output_scale,
-                    aa_scale,
+                    animation.origin,
+                    animation.offset,
+                    animation.scale,
                 ),
-                Point::<i32, Physical>::from((0, 0)),
-                zoom,
-            ),
-        ));
+            ));
+        } else {
+            target.push(OutputRenderElements::CsdWindow(elem));
+        }
     }
 }
 
@@ -143,13 +164,21 @@ fn push_plain_elements(
     target: &mut Vec<OutputRenderElements>,
     elems: Vec<WaylandSurfaceRenderElement<GlesRenderer>>,
     zoom: f64,
+    animation: Option<WindowRenderAnimation>,
 ) {
     target.extend(elems.into_iter().map(|elem| {
-        OutputRenderElements::Window(PixelSnapRescaleElement::from_element(
-            elem,
-            Point::<i32, Physical>::from((0, 0)),
-            zoom,
-        ))
+        let elem =
+            PixelSnapRescaleElement::from_element(elem, Point::<i32, Physical>::from((0, 0)), zoom);
+        if let Some(animation) = animation {
+            OutputRenderElements::AnimatedWindow(WindowTransformElement::new(
+                elem,
+                animation.origin,
+                animation.offset,
+                animation.scale,
+            ))
+        } else {
+            OutputRenderElements::Window(elem)
+        }
     }));
 }
 
@@ -335,7 +364,7 @@ pub(crate) fn compose_capture_elements(
 
         let target = if is_widget { &mut widgets } else { &mut normal };
         // Popups push first so they sit above the title bar and window content.
-        push_plain_elements(target, popup_elems, zoom);
+        push_plain_elements(target, popup_elems, zoom, None);
 
         if has_ssd {
             let bar_height = state.config.decorations.title_bar_height;
@@ -395,12 +424,13 @@ pub(crate) fn compose_capture_elements(
                         [0.0, 0.0, radius, radius],
                         zoom,
                         output_scale,
+                        None,
                     );
                 } else {
-                    push_plain_elements(target, elems, zoom);
+                    push_plain_elements(target, elems, zoom, None);
                 }
             } else {
-                push_plain_elements(target, elems, zoom);
+                push_plain_elements(target, elems, zoom, None);
             }
 
             if effective_bw > 0
@@ -423,6 +453,7 @@ pub(crate) fn compose_capture_elements(
                     opacity,
                     scale,
                     zoom,
+                    None,
                 );
             }
 
@@ -446,6 +477,7 @@ pub(crate) fn compose_capture_elements(
                     opacity,
                     scale,
                     zoom,
+                    None,
                 );
             }
         } else if let Some(ref shader) = state.render.corner_clip_shader {
@@ -469,6 +501,7 @@ pub(crate) fn compose_capture_elements(
                     [radius, radius, radius, radius],
                     zoom,
                     output_scale,
+                    None,
                 );
 
                 if effective_bw > 0
@@ -487,6 +520,7 @@ pub(crate) fn compose_capture_elements(
                         opacity,
                         scale,
                         zoom,
+                        None,
                     );
                 }
 
@@ -510,13 +544,14 @@ pub(crate) fn compose_capture_elements(
                         opacity,
                         scale,
                         zoom,
+                        None,
                     );
                 }
             } else {
-                push_plain_elements(target, elems, zoom);
+                push_plain_elements(target, elems, zoom, None);
             }
         } else {
-            push_plain_elements(target, elems, zoom);
+            push_plain_elements(target, elems, zoom, None);
         }
     }
 
@@ -561,7 +596,7 @@ pub fn compose_frame(
     }
 
     let name = output.name();
-    let output_fullscreen = state.is_output_fullscreen(output);
+    let output_fullscreen = state.is_output_visually_fullscreen(output);
     // The fullscreen window fully occludes its output: only it, the overlay
     // layer, and the cursor render; everything beneath is culled below. Pinned
     // windows count as top-tier toplevels and get covered like the top layer.
@@ -615,6 +650,7 @@ pub fn compose_frame(
     // Screen-pinned windows: own bucket, rendered above normal and below
     // Top/Overlay layer-shell (see all_elements assembly below).
     let mut zoomed_pinned: Vec<OutputRenderElements> = Vec::new();
+    let mut completed_closes: Vec<(smithay::desktop::Window, Option<ClosingSnapshot>)> = Vec::new();
 
     let blur_enabled = state.render.blur_down_shader.is_some()
         && state.render.blur_up_shader.is_some()
@@ -712,6 +748,28 @@ pub fn compose_frame(
         else {
             continue;
         };
+        let visual = state.window_visual(window, loc, geom_size);
+        let target_size = geom_size.to_f64();
+        let animated =
+            visual.loc != loc.to_f64() || visual.size != target_size || visual.alpha != 1.0;
+        let window_animation = animated.then(|| {
+            let physical_zoom = output_scale * zoom;
+            let content_origin = Point::from((
+                (render_loc.x + geom_loc.x as f64) * physical_zoom,
+                (render_loc.y + geom_loc.y as f64) * physical_zoom,
+            ));
+            WindowRenderAnimation {
+                origin: content_origin,
+                offset: Point::from((
+                    (visual.loc.x - loc.x as f64) * physical_zoom,
+                    (visual.loc.y - loc.y as f64) * physical_zoom,
+                )),
+                scale: Scale::from((
+                    visual.size.w / target_size.w.max(1.0),
+                    visual.size.h / target_size.h.max(1.0),
+                )),
+            }
+        });
 
         #[cfg(feature = "profile-with-tracy")]
         {
@@ -726,7 +784,7 @@ pub fn compose_frame(
         // Empty rect list = client explicitly opted out → treat as off.
         let client_blur = client_blur_rects.as_ref().is_some_and(|r| !r.is_empty());
         let wants_blur = blur_enabled && (applied.as_ref().is_some_and(|r| r.blur) || client_blur);
-        let opacity = applied.as_ref().and_then(|r| r.opacity).unwrap_or(1.0);
+        let opacity = applied.as_ref().and_then(|r| r.opacity).unwrap_or(1.0) * visual.alpha as f64;
 
         // Split elements: toplevel + subsurfaces get corner-clipped, popups
         // don't (they can legitimately extend outside the parent's geometry —
@@ -784,7 +842,7 @@ pub fn compose_frame(
 
         // Popups push first (earlier in vec = on-top in smithay z-order) so
         // they sit above the title bar and clipped window content.
-        push_plain_elements(target, popup_elems, zoom);
+        push_plain_elements(target, popup_elems, zoom, window_animation);
 
         if has_ssd {
             let bar_height = state.config.decorations.title_bar_height;
@@ -858,12 +916,13 @@ pub fn compose_frame(
                         [0.0, 0.0, radius, radius],
                         zoom,
                         output_scale,
+                        window_animation,
                     );
                 } else {
-                    push_plain_elements(target, elems, zoom);
+                    push_plain_elements(target, elems, zoom, window_animation);
                 }
             } else {
-                push_plain_elements(target, elems, zoom);
+                push_plain_elements(target, elems, zoom, window_animation);
             }
 
             // Border wraps title bar + content; drawn between window content
@@ -888,6 +947,7 @@ pub fn compose_frame(
                     opacity,
                     scale,
                     zoom,
+                    window_animation,
                 );
             }
 
@@ -915,6 +975,7 @@ pub fn compose_frame(
                     opacity,
                     scale,
                     zoom,
+                    window_animation,
                 );
                 shadow_count = 1;
             }
@@ -951,6 +1012,7 @@ pub fn compose_frame(
                     [radius, radius, radius, radius],
                     zoom,
                     output_scale,
+                    window_animation,
                 );
 
                 if effective_bw > 0
@@ -969,6 +1031,7 @@ pub fn compose_frame(
                         opacity,
                         scale,
                         zoom,
+                        window_animation,
                     );
                 }
 
@@ -994,15 +1057,16 @@ pub fn compose_frame(
                         opacity,
                         scale,
                         zoom,
+                        window_animation,
                     );
                     shadow_count = 1;
                 }
             } else {
                 // Bare (`decoration = "none"`) or fullscreen: pass through.
-                push_plain_elements(target, elems, zoom);
+                push_plain_elements(target, elems, zoom, window_animation);
             }
         } else {
-            push_plain_elements(target, elems, zoom);
+            push_plain_elements(target, elems, zoom, window_animation);
         }
 
         if wants_blur && (target.len() - elem_start - shadow_count) > 0 {
@@ -1103,8 +1167,42 @@ pub fn compose_frame(
                 });
             }
         }
+
+        if state.window_close_pending(window) {
+            let snapshot = match closing::capture(
+                renderer,
+                &output.name(),
+                scale,
+                camera,
+                zoom,
+                is_pinned,
+                &target[elem_start..],
+            ) {
+                Ok(snapshot) => snapshot,
+                Err(err) => {
+                    tracing::warn!("failed to snapshot closing window: {err}");
+                    None
+                }
+            };
+            completed_closes.push((window.clone(), snapshot));
+        }
     }
 
+    // Mutating the stage while iterating it would invalidate the z-order
+    // iterator. Finalize closes only after every live window was composed.
+    for (window, snapshot) in completed_closes {
+        state.finish_snapshotted_close(&window, snapshot);
+    }
+    let mut closing_elements = closing::render_for_output(
+        &state.closing_snapshots,
+        &output.name(),
+        camera,
+        zoom,
+        output_scale,
+    );
+    closing_elements.extend(zoomed_normal);
+    zoomed_normal = closing_elements;
+
     #[cfg(feature = "profile-with-tracy")]
     {
         static VISIBLE_PLOT: std::sync::OnceLock<tracy_client::PlotName> =
@@ -1174,7 +1272,7 @@ pub fn compose_frame(
         vec![]
     };
 
-    let is_fullscreen = state.is_output_fullscreen(output);
+    let is_fullscreen = state.is_output_visually_fullscreen(output);
     #[cfg(feature = "profile-with-tracy")]
     let _layers_span = tracy_client::span!("compose::layers");
     let (overlay_elements, overlay_blur) = build_layer_elements(
diff --git a/src/render/shaders.rs b/src/render/shaders.rs
index 88a1f77..d35ca50 100644
--- a/src/render/shaders.rs
+++ b/src/render/shaders.rs
@@ -182,6 +182,7 @@ pub(super) fn push_shadow_element(
     opacity: f64,
     output_scale: Scale<f64>,
     zoom: f64,
+    animation: Option<super::WindowRenderAnimation>,
 ) {
     use driftwm::config::DecorationConfig;
     let shadow_radius = DecorationConfig::SHADOW_RADIUS;
@@ -233,13 +234,23 @@ pub(super) fn push_shadow_element(
         elem.update_uniforms(fresh_uniforms);
     }
     elem.resize(shadow_area, None);
-    target.push(OutputRenderElements::Background(
-        RescaleRenderElement::from_element(
-            elem.clone(),
-            Point::<i32, Physical>::from((0, 0)),
-            zoom,
-        ),
-    ));
+    let elem = RescaleRenderElement::from_element(
+        elem.clone(),
+        Point::<i32, Physical>::from((0, 0)),
+        zoom,
+    );
+    if let Some(animation) = animation {
+        target.push(OutputRenderElements::AnimatedChrome(
+            super::WindowTransformElement::new(
+                elem,
+                animation.origin,
+                animation.offset,
+                animation.scale,
+            ),
+        ));
+    } else {
+        target.push(OutputRenderElements::Background(elem));
+    }
 }
 
 const BORDER_SHADER_SRC: &str = include_str!("../shaders/border.glsl");
@@ -370,6 +381,7 @@ pub(super) fn push_border_element(
     opacity: f64,
     output_scale: Scale<f64>,
     zoom: f64,
+    animation: Option<super::WindowRenderAnimation>,
 ) {
     if border_width_logical <= 0 {
         return;
@@ -434,13 +446,23 @@ pub(super) fn push_border_element(
         elem.update_uniforms(fresh_uniforms);
     }
     elem.resize(border_area, None);
-    target.push(OutputRenderElements::Background(
-        RescaleRenderElement::from_element(
-            elem.clone(),
-            Point::<i32, Physical>::from((0, 0)),
-            zoom,
-        ),
-    ));
+    let elem = RescaleRenderElement::from_element(
+        elem.clone(),
+        Point::<i32, Physical>::from((0, 0)),
+        zoom,
+    );
+    if let Some(animation) = animation {
+        target.push(OutputRenderElements::AnimatedChrome(
+            super::WindowTransformElement::new(
+                elem,
+                animation.origin,
+                animation.offset,
+                animation.scale,
+            ),
+        ));
+    } else {
+        target.push(OutputRenderElements::Background(elem));
+    }
 }
 
 const CORNER_CLIP_SRC: &str = include_str!("../shaders/corner_clip.glsl");
diff --git a/src/stage/mod.rs b/src/stage/mod.rs
index 805c759..85e3020 100644
--- a/src/stage/mod.rs
+++ b/src/stage/mod.rs
@@ -53,6 +53,10 @@ struct Entry<W> {
     /// geometry because some clients (Chromium) shrink their reported
     /// geometry after each sized configure.
     restore_size: Option<Size<i32, Logical>>,
+    /// `Some((pre-fill position, pre-fill size))` while the window is filled.
+    /// Unlike fit, fill restores position too — a filled window grows in place
+    /// rather than centering, so the exact origin must round-trip.
+    fill_saved: Option<(Point<i32, Logical>, Size<i32, Logical>)>,
     /// `Some` while the window is pinned to an output's screen space.
     pinned: Option<PinnedSite>,
 }
@@ -110,6 +114,7 @@ impl<W: StageElement> Stage<W> {
                 position,
                 fit_saved_size: None,
                 restore_size: None,
+                fill_saved: None,
                 pinned: None,
             });
         }
@@ -370,6 +375,37 @@ impl<W: StageElement> Stage<W> {
         }
     }
 
+    /// Mark `window` filled, saving its pre-fill position and size for restore.
+    pub fn set_fill(
+        &mut self,
+        window: &W,
+        saved_position: Point<i32, Logical>,
+        saved_size: Size<i32, Logical>,
+    ) {
+        if let Some(e) = self.entry_mut(window) {
+            e.fill_saved = Some((saved_position, saved_size));
+        }
+    }
+
+    pub fn is_fill(&self, window: &W) -> bool {
+        self.entry(window).is_some_and(|e| e.fill_saved.is_some())
+    }
+
+    /// Clear fill state, returning the saved pre-fill position and size (the
+    /// unfill path).
+    pub fn take_fill_saved(
+        &mut self,
+        window: &W,
+    ) -> Option<(Point<i32, Logical>, Size<i32, Logical>)> {
+        self.entry_mut(window).and_then(|e| e.fill_saved.take())
+    }
+
+    pub fn clear_fill(&mut self, window: &W) {
+        if let Some(e) = self.entry_mut(window) {
+            e.fill_saved = None;
+        }
+    }
+
     pub fn restore_size(&self, window: &W) -> Option<Size<i32, Logical>> {
         self.entry(window).and_then(|e| e.restore_size)
     }
@@ -476,6 +512,12 @@ impl<W: StageElement> Stage<W> {
                     "fit window has empty saved size"
                 );
             }
+            if let Some((_, saved)) = e.fill_saved {
+                assert!(
+                    saved.w > 0 && saved.h > 0,
+                    "fill window has empty saved size"
+                );
+            }
             if e.pinned.is_some() {
                 assert!(
                     !self.focus_history.contains(&e.window),
diff --git a/src/stage/tests.rs b/src/stage/tests.rs
index dbbbe2a..f70e6a8 100644
--- a/src/stage/tests.rs
+++ b/src/stage/tests.rs
@@ -532,6 +532,9 @@ mod harness {
         ToggleFit {
             idx: usize,
         },
+        ToggleFillMembership {
+            idx: usize,
+        },
         ResizeGrabEnd {
             idx: usize,
             w: i32,
@@ -579,6 +582,7 @@ mod harness {
             1 => (0..3usize).prop_map(|output| Op::RemoveOutput { output }),
             1 => (0..3usize).prop_map(|output| Op::AddOutput { output }),
             2 => idx.clone().prop_map(|idx| Op::ToggleFit { idx }),
+            1 => idx.clone().prop_map(|idx| Op::ToggleFillMembership { idx }),
             1 => (idx.clone(), 50..500i32, 50..500i32)
                 .prop_map(|(idx, w, h)| Op::ResizeGrabEnd { idx, w, h }),
             2 => (idx.clone(), -300..300i32, -300..300i32)
@@ -1246,6 +1250,27 @@ mod harness {
                         self.raise_and_focus(&w);
                     }
                 }
+                Op::ToggleFillMembership { idx } => {
+                    // Fill's geometry lives in DriftWm, not the stage; here we
+                    // only exercise the membership field so verify_invariants
+                    // covers it. Same eligibility as the keybinding path.
+                    let Some(w) = self.pick(*idx) else { return };
+                    if w.is_widget()
+                        || self.stage.is_fullscreen(&w)
+                        || self.stage.is_pinned(&w)
+                        || self.stage.is_fit(&w)
+                        || !self.stage.contains(&w)
+                    {
+                        return;
+                    }
+                    if self.stage.is_fill(&w) {
+                        self.stage.take_fill_saved(&w);
+                    } else {
+                        let pos = self.stage.position_of(&w).unwrap_or_default();
+                        let size = StageElement::size(&w);
+                        self.stage.set_fill(&w, pos, size);
+                    }
+                }
                 Op::ResizeGrabEnd { idx, w: nw, h: nh } => {
                     // End of a user resize: fit cleared at grab start, size
                     // committed, restore size anchored to the user's choice.
diff --git a/src/state/animation.rs b/src/state/animation.rs
index 79ac73c..bc86c97 100644
--- a/src/state/animation.rs
+++ b/src/state/animation.rs
@@ -1,9 +1,12 @@
 use std::time::{Duration, Instant};
 
 use smithay::input::pointer::CursorImageStatus;
+use smithay::reexports::wayland_server::Resource;
 use smithay::utils::{Logical, Point};
+use smithay::wayland::seat::WaylandFocus;
 
 use driftwm::canvas::{self, CanvasPos};
+use driftwm::window_ext::WindowExt;
 use smithay::wayland::shell::wlr_layer::Layer as WlrLayer;
 
 use smithay::output::Output;
@@ -19,6 +22,200 @@ impl DriftWm {
         1.0 - (1.0 - base).powf(dt_secs * 60.0)
     }
 
+    pub(crate) fn start_window_open_animation(&mut self, window: &smithay::desktop::Window) {
+        if self.backend.is_none() {
+            return;
+        }
+        self.window_animations.start_open(window);
+        self.mark_all_dirty();
+    }
+
+    pub(crate) fn animate_window_geometry(
+        &mut self,
+        window: &smithay::desktop::Window,
+        to_loc: Point<i32, Logical>,
+        to_size: smithay::utils::Size<i32, Logical>,
+    ) {
+        if self.backend.is_none() {
+            return;
+        }
+        let Some(from_loc) = self.stage.position_of(window) else {
+            return;
+        };
+        let from_size = window.geometry().size;
+        // A new action may interrupt an animation whose stage geometry already
+        // points at its old destination. Continue from what is actually shown,
+        // not from that logical destination, so rapid toggles reverse cleanly.
+        let visual = self.window_visual(window, from_loc, from_size);
+        if visual.loc == to_loc.to_f64() && visual.size == to_size.to_f64() {
+            return;
+        }
+        self.window_animations.start_geometry(
+            window,
+            visual.loc.to_i32_round(),
+            visual.size.to_i32_round(),
+            to_loc,
+            to_size,
+        );
+        self.mark_all_dirty();
+    }
+
+    pub(crate) fn animate_window_geometry_from(
+        &mut self,
+        window: &smithay::desktop::Window,
+        from_loc: Point<i32, Logical>,
+        to_loc: Point<i32, Logical>,
+    ) {
+        if self.backend.is_none() || from_loc == to_loc {
+            return;
+        }
+        let size = window.geometry().size;
+        self.window_animations
+            .start_geometry(window, from_loc, size, to_loc, size);
+        self.mark_all_dirty();
+    }
+
+    pub(crate) fn animate_window_geometry_between(
+        &mut self,
+        window: &smithay::desktop::Window,
+        from_loc: Point<i32, Logical>,
+        from_size: smithay::utils::Size<i32, Logical>,
+        to_loc: Point<i32, Logical>,
+        to_size: smithay::utils::Size<i32, Logical>,
+    ) {
+        if self.backend.is_none() {
+            return;
+        }
+        self.window_animations
+            .start_geometry(window, from_loc, from_size, to_loc, to_size);
+        self.mark_all_dirty();
+    }
+
+    pub(crate) fn animate_window_fullscreen(
+        &mut self,
+        window: &smithay::desktop::Window,
+        from_loc: Point<i32, Logical>,
+        from_size: smithay::utils::Size<i32, Logical>,
+        to_loc: Point<i32, Logical>,
+        to_size: smithay::utils::Size<i32, Logical>,
+    ) {
+        if self.backend.is_none() {
+            return;
+        }
+        self.window_animations
+            .start_fullscreen(window, from_loc, from_size, to_loc, to_size);
+        self.mark_all_dirty();
+    }
+
+    pub(crate) fn window_fullscreen_animation_active(
+        &self,
+        window: &smithay::desktop::Window,
+    ) -> bool {
+        self.window_animations.is_fullscreen_transition(window)
+    }
+
+    pub fn request_window_close(&mut self, window: &smithay::desktop::Window) {
+        if self.backend.is_none() {
+            window.send_close();
+            return;
+        }
+        if !self.window_animations.request_close(window) {
+            return;
+        }
+        let can_capture = matches!(self.session_lock, super::SessionLock::Unlocked)
+            && self
+                .space
+                .outputs()
+                .filter(|output| {
+                    self.active_outputs.contains(*output)
+                        && !self.dpms_off_outputs.contains(*output)
+                })
+                .cloned()
+                .collect::<Vec<_>>()
+                .into_iter()
+                .any(|output| self.window_intersects_viewport_on(window, &output));
+        if can_capture {
+            self.mark_all_dirty();
+        } else {
+            // Nothing can flash on screen, and no output composition pass can
+            // capture this window. Remove it immediately without a snapshot.
+            self.finish_snapshotted_close(window, None);
+        }
+    }
+
+    pub(crate) fn tick_window_animations(&mut self, dt: Duration) {
+        let speed = self.config.animation_speed;
+        self.window_animations.tick(dt, speed);
+        let frame_factor = 1.0 - (1.0 - speed).powf(dt.as_secs_f64() * 60.0);
+        for snapshot in &mut self.closing_snapshots {
+            snapshot.tick(frame_factor);
+        }
+        self.closing_snapshots
+            .retain(|snapshot| !snapshot.is_done());
+    }
+
+    pub(crate) fn window_close_pending(&self, window: &smithay::desktop::Window) -> bool {
+        window
+            .wl_surface()
+            .is_some_and(|surface| self.window_animations.close_pending(&surface.id()))
+    }
+
+    pub(crate) fn finish_snapshotted_close(
+        &mut self,
+        window: &smithay::desktop::Window,
+        snapshot: Option<crate::render::ClosingSnapshot>,
+    ) {
+        let Some(surface) = window.wl_surface() else {
+            return;
+        };
+        let Some(close_window) = self.window_animations.take_pending_close(&surface.id()) else {
+            return;
+        };
+        if let Some(snapshot) = snapshot {
+            self.closing_snapshots.push(snapshot);
+        }
+
+        let was_focused = self.focused_window().as_ref() == Some(window);
+        self.unmap_window(window);
+        close_window.send_close();
+
+        if was_focused {
+            let next = self.stage.focus_history().first().cloned();
+            if let Some(next) = next {
+                if self.config.auto_navigate_on_close {
+                    self.navigate_to_window(&next, false);
+                } else if self.window_fully_in_viewport(&next) {
+                    let serial = smithay::utils::SERIAL_COUNTER.next_serial();
+                    self.raise_and_focus(&next, serial);
+                } else {
+                    let serial = smithay::utils::SERIAL_COUNTER.next_serial();
+                    self.set_window_focus(None, serial);
+                }
+            } else {
+                let serial = smithay::utils::SERIAL_COUNTER.next_serial();
+                self.set_window_focus(None, serial);
+            }
+        }
+        self.refresh_pointer_focus();
+    }
+
+    pub(crate) fn window_visual(
+        &self,
+        window: &smithay::desktop::Window,
+        target_loc: Point<i32, Logical>,
+        target_size: smithay::utils::Size<i32, Logical>,
+    ) -> super::window_animation::WindowVisual {
+        let Some(surface) = window.wl_surface() else {
+            return super::window_animation::WindowVisual {
+                loc: target_loc.to_f64(),
+                size: target_size.to_f64(),
+                alpha: 1.0,
+            };
+        };
+        self.window_animations
+            .visual(&surface.id(), target_loc, target_size)
+    }
+
     /// Fire held compositor action if repeat delay/rate has elapsed.
     pub fn apply_key_repeat(&mut self) {
         let Some((_, ref action, next_fire)) = self.held_action else {
@@ -243,7 +440,7 @@ impl DriftWm {
         self.with_output_state(|os| {
             os.camera_target = None;
             os.zoom_target = None;
-            os.zoom_animation_center = None;
+            os.zoom_animation_anchor = None;
             os.overview_return = None;
             os.momentum.accumulate(delta, time_ms);
             os.camera.x += delta.x;
@@ -265,7 +462,7 @@ impl DriftWm {
             let mut os = super::output_state(output);
             os.camera_target = None;
             os.zoom_target = None;
-            os.zoom_animation_center = None;
+            os.zoom_animation_anchor = None;
             os.overview_return = None;
             os.momentum.accumulate(delta, time_ms);
             os.camera.x += delta.x;
@@ -367,8 +564,8 @@ impl DriftWm {
     }
 
     /// Advance zoom animation toward `zoom_target` using frame-rate independent lerp.
-    /// When `zoom_animation_center` is set (combined zoom+camera animation), lerps
-    /// the on-screen center directly and derives camera, preventing lateral drift.
+    /// When `zoom_animation_anchor` is set (combined zoom+camera animation), keeps
+    /// its screen-space anchor stable while deriving camera, preventing drift.
     /// Otherwise just adjusts pointer so cursor stays at the same screen position.
     pub fn apply_zoom_animation(&mut self, dt: Duration) {
         let Some(target) = self.zoom_target() else {
@@ -381,39 +578,52 @@ impl DriftWm {
         let factor = self.animation_factor(dt);
 
         let dz = target - old_zoom;
-        if dz.abs() < 0.001 {
+        let zoom_close = dz.abs() < 0.001;
+        if zoom_close {
             self.set_zoom(target);
-            self.set_zoom_target(None);
+            if self.zoom_animation_anchor().is_none() {
+                self.set_zoom_target(None);
+            }
         } else {
             self.set_zoom(old_zoom + dz * factor);
         }
 
-        if let Some(target_center) = self.zoom_animation_center() {
-            // Combined zoom+camera: lerp the on-screen center, derive camera
-            let vc = self.usable_center_screen();
-            let current_center: Point<f64, Logical> = Point::from((
-                old_camera.x + vc.x / old_zoom,
-                old_camera.y + vc.y / old_zoom,
+        if let Some(anchor) = self.zoom_animation_anchor() {
+            // Combined zoom+camera: lerp the canvas point at the fixed screen
+            // anchor, then derive camera. The anchor can be the viewport center
+            // (keyboard/fit) or the pointer position (wheel zoom).
+            let current_anchor: Point<f64, Logical> = Point::from((
+                old_camera.x + anchor.screen.x / old_zoom,
+                old_camera.y + anchor.screen.y / old_zoom,
             ));
-            let cx = current_center.x + (target_center.x - current_center.x) * factor;
-            let cy = current_center.y + (target_center.y - current_center.y) * factor;
+            let cx = current_anchor.x + (anchor.canvas.x - current_anchor.x) * factor;
+            let cy = current_anchor.y + (anchor.canvas.y - current_anchor.y) * factor;
 
             let cur_zoom = self.zoom();
-            self.set_camera(Point::from((cx - vc.x / cur_zoom, cy - vc.y / cur_zoom)));
+            self.set_camera(Point::from((
+                cx - anchor.screen.x / cur_zoom,
+                cy - anchor.screen.y / cur_zoom,
+            )));
             self.update_output_from_camera();
 
             // Suppress camera_animation — we set camera directly
             self.set_camera_target(None);
 
-            if self.zoom_target().is_none() {
-                // Zoom snapped — hand off final convergence to camera_animation
+            let center_dx = anchor.canvas.x - current_anchor.x;
+            let center_dy = anchor.canvas.y - current_anchor.y;
+            if zoom_close && center_dx * center_dx + center_dy * center_dy < 0.25 {
+                // Finish both coordinates together. Keeping one coupled
+                // animation avoids the camera-only tail that made zoom-to-fit
+                // change velocity near the end.
                 let cur_zoom = self.zoom();
                 let final_camera = Point::from((
-                    target_center.x - vc.x / cur_zoom,
-                    target_center.y - vc.y / cur_zoom,
+                    anchor.canvas.x - anchor.screen.x / cur_zoom,
+                    anchor.canvas.y - anchor.screen.y / cur_zoom,
                 ));
-                self.set_zoom_animation_center(None);
-                self.set_camera_target(Some(final_camera));
+                self.set_zoom_target(None);
+                self.clear_zoom_animation_anchor();
+                self.set_camera(final_camera);
+                self.update_output_from_camera();
             }
 
             // Warp pointer: compensate for both camera and zoom change
@@ -457,6 +667,7 @@ impl DriftWm {
         // Global (not per-output) ticks
         self.apply_key_repeat();
         self.check_exec_cursor_timeout();
+        self.tick_window_animations(dt);
         // Re-arm cursor edge-pan from the current cursor position before the
         // per-output velocities are applied below (disarms outputs the cursor
         // has left; keeps the active output's speed stable frame-to-frame).
@@ -482,7 +693,7 @@ impl DriftWm {
                 let mut os = output_state(output);
                 os.camera_target = None;
                 os.zoom_target = None;
-                os.zoom_animation_center = None;
+                os.zoom_animation_anchor = None;
             }
             self.tick_zoom_animation_on(output, is_active, dt);
             self.tick_camera_animation_on(output, is_active, dt);
@@ -572,54 +783,61 @@ impl DriftWm {
     }
 
     fn tick_zoom_animation_on(&mut self, output: &Output, is_active: bool, dt: Duration) {
-        let (target, old_zoom, old_camera, anim_center) = {
+        let (target, old_zoom, old_camera, anim_anchor) = {
             let os = output_state(output);
             let Some(target) = os.zoom_target else { return };
-            (target, os.zoom, os.camera, os.zoom_animation_center)
+            (target, os.zoom, os.camera, os.zoom_animation_anchor)
         };
 
         let factor = self.animation_factor(dt);
 
         let dz = target - old_zoom;
+        let zoom_close = dz.abs() < 0.001;
         {
             let mut os = output_state(output);
-            if dz.abs() < 0.001 {
+            if zoom_close {
                 os.zoom = target;
-                os.zoom_target = None;
+                if anim_anchor.is_none() {
+                    os.zoom_target = None;
+                }
                 drop(os);
             } else {
                 os.zoom = old_zoom + dz * factor;
             }
         }
 
-        if let Some(target_center) = anim_center {
-            let vc = super::usable_center_for_output(output);
-
-            let current_center: Point<f64, Logical> = Point::from((
-                old_camera.x + vc.x / old_zoom,
-                old_camera.y + vc.y / old_zoom,
+        if let Some(anchor) = anim_anchor {
+            let current_anchor: Point<f64, Logical> = Point::from((
+                old_camera.x + anchor.screen.x / old_zoom,
+                old_camera.y + anchor.screen.y / old_zoom,
             ));
-            let cx = current_center.x + (target_center.x - current_center.x) * factor;
-            let cy = current_center.y + (target_center.y - current_center.y) * factor;
+            let cx = current_anchor.x + (anchor.canvas.x - current_anchor.x) * factor;
+            let cy = current_anchor.y + (anchor.canvas.y - current_anchor.y) * factor;
 
             let cur_zoom = output_state(output).zoom;
             self.set_camera_on(
                 output,
-                Point::from((cx - vc.x / cur_zoom, cy - vc.y / cur_zoom)),
+                Point::from((
+                    cx - anchor.screen.x / cur_zoom,
+                    cy - anchor.screen.y / cur_zoom,
+                )),
             );
             {
                 let mut os = output_state(output);
                 // Suppress camera_animation — we set camera directly
                 os.camera_target = None;
 
-                if os.zoom_target.is_none() {
-                    // Zoom snapped — hand off final convergence to camera_animation
+                let center_dx = anchor.canvas.x - current_anchor.x;
+                let center_dy = anchor.canvas.y - current_anchor.y;
+                if zoom_close && center_dx * center_dx + center_dy * center_dy < 0.25 {
                     let final_camera = Point::from((
-                        target_center.x - vc.x / cur_zoom,
-                        target_center.y - vc.y / cur_zoom,
+                        anchor.canvas.x - anchor.screen.x / cur_zoom,
+                        anchor.canvas.y - anchor.screen.y / cur_zoom,
                     ));
-                    os.zoom_animation_center = None;
-                    os.camera_target = Some(final_camera);
+                    os.zoom_target = None;
+                    os.zoom_animation_anchor = None;
+                    drop(os);
+                    self.set_camera_on(output, final_camera);
                 }
             }
 
diff --git a/src/state/cluster_snapshot.rs b/src/state/cluster_snapshot.rs
index a1fc6f9..c0dac75 100644
--- a/src/state/cluster_snapshot.rs
+++ b/src/state/cluster_snapshot.rs
@@ -100,6 +100,8 @@ impl ClusterResizeSnapshot {
             if !m.window.alive() {
                 continue;
             }
+            // Shifting a member re-anchors it, invalidating any fill restore point.
+            stage.clear_fill(&m.window);
             let new_pos = m.initial_pos + Point::from((*dx, *dy));
             stage.map(m.window.clone(), new_pos);
         }
diff --git a/src/state/fill.rs b/src/state/fill.rs
new file mode 100644
index 0000000..572f767
--- /dev/null
+++ b/src/state/fill.rs
@@ -0,0 +1,221 @@
+use smithay::{
+    desktop::Window,
+    reexports::wayland_server::Resource,
+    utils::{Logical, Point, Size},
+    wayland::seat::WaylandFocus,
+};
+
+use super::{DriftWm, PendingRecenter, output_state};
+use crate::grabs::SizeConstraints;
+use driftwm::canvas::{ScreenPos, screen_to_canvas};
+use driftwm::config;
+use driftwm::layout::snap::SnapRect;
+
+/// The window's target content size + map location after filling the free space
+/// around it, or `None` when filling is a no-op (already fills the space, or the
+/// window sits entirely outside the usable area).
+struct FillGeometry {
+    new_loc: Point<i32, Logical>,
+    new_size: Size<i32, Logical>,
+    frame: SnapRect,
+}
+
+impl DriftWm {
+    fn compute_fill_geometry(&self, window: &Window) -> Option<FillGeometry> {
+        let surface = window.wl_surface()?;
+        let output = self.output_for_window(window)?;
+
+        // Usable screen rect → canvas rect via the output's own camera/zoom.
+        let usable = self.usable_area_on(&output);
+        let (camera, zoom) = {
+            let os = output_state(&output);
+            (os.camera, os.zoom)
+        };
+        let top_left = screen_to_canvas(
+            ScreenPos(Point::from((usable.loc.x as f64, usable.loc.y as f64))),
+            camera,
+            zoom,
+        )
+        .0;
+        let bottom_right = screen_to_canvas(
+            ScreenPos(Point::from((
+                (usable.loc.x + usable.size.w) as f64,
+                (usable.loc.y + usable.size.h) as f64,
+            ))),
+            camera,
+            zoom,
+        )
+        .0;
+        let bounds = SnapRect {
+            x_low: top_left.x,
+            x_high: bottom_right.x,
+            y_low: top_left.y,
+            y_high: bottom_right.y,
+        };
+
+        let current = self.snap_rect_for(window)?;
+        let obstacles: Vec<SnapRect> = self
+            .all_windows_with_snap_rects()
+            .into_iter()
+            .filter(|(w, _)| w != window)
+            .map(|(_, r)| r)
+            .collect();
+
+        // `window_snap_rect` inflates content by the SSD bar (top) and border
+        // (all sides); mirror that inflation onto the client's content-space
+        // size hints so the constraints live in the same frame space as the
+        // rects, preserving the 0 = unconstrained sentinel.
+        let bar = self.window_ssd_bar(window);
+        let bw = self.window_border_width(&surface);
+        let inflate = |v: i32, extra: i32| -> f64 { if v > 0 { (v + extra) as f64 } else { 0.0 } };
+        let constraints = SizeConstraints::for_window(window);
+        let min_size = (
+            inflate(constraints.min.w, 2 * bw),
+            inflate(constraints.min.h, 2 * bw + bar),
+        );
+        let max_size = (
+            inflate(constraints.max.w, 2 * bw),
+            inflate(constraints.max.h, 2 * bw + bar),
+        );
+
+        let filled = driftwm::layout::fill::fill_rect(
+            current,
+            &obstacles,
+            bounds,
+            self.config.snap_gap,
+            min_size,
+            max_size,
+        )?;
+
+        // Invert `window_snap_rect` back to a content size + top-left location.
+        let bw = bw as f64;
+        let bar = bar as f64;
+        // Deflating a sliver free-region by borders/bar can go non-positive; a
+        // client size must stay at least 1px on each axis.
+        let new_size = Size::from((
+            ((filled.x_high - filled.x_low - 2.0 * bw).round() as i32).max(1),
+            ((filled.y_high - filled.y_low - 2.0 * bw - bar).round() as i32).max(1),
+        ));
+        let new_loc = Point::from((
+            (filled.x_low + bw).round() as i32,
+            (filled.y_low + bar + bw).round() as i32,
+        ));
+
+        // No-op: the window already fills its free space. Return without
+        // committing so `fill_window` won't record a restore point.
+        let cur_loc = self.stage.position_of(window)?;
+        if new_size == window.geometry().size && new_loc == cur_loc {
+            return None;
+        }
+        Some(FillGeometry {
+            new_loc,
+            new_size,
+            frame: filled,
+        })
+    }
+
+    pub fn fill_window(&mut self, window: &Window) {
+        let Some(wl_surface) = window.wl_surface() else {
+            return;
+        };
+        // A fit (maximized) window is fit's business; a widget or pinned window
+        // has no free canvas space to grow into.
+        if self.is_pinned(window)
+            || self.stage.is_fit(window)
+            || config::applied_rule(&wl_surface).is_some_and(|r| r.widget)
+        {
+            return;
+        }
+
+        let Some(FillGeometry {
+            new_loc,
+            new_size,
+            frame,
+        }) = self.compute_fill_geometry(window)
+        else {
+            return;
+        };
+
+        // Use the tracked restore size rather than window.geometry().size — for
+        // Chromium the latter shrinks on each round-trip (see fit_window).
+        let saved_size = self
+            .stage
+            .restore_size(window)
+            .unwrap_or_else(|| window.geometry().size);
+        let Some(saved_pos) = self.stage.position_of(window) else {
+            return;
+        };
+
+        self.animate_window_geometry(window, new_loc, new_size);
+        self.send_size_configure(window, new_size);
+        self.map_window(window.clone(), new_loc, false);
+        self.stage.set_fill(window, saved_pos, saved_size);
+        // Cache the filled rect directly: `geometry().size` is still pre-ack, so
+        // `refresh_stable_snap_rect` would cache stale dimensions. Unlike plain
+        // fit, the filled rect is the window's new in-place identity — leaving
+        // the pre-fill rect cached makes every later commit read as "grew past
+        // settled" (a perpetual reflow scan once something clears the fill
+        // state, and a reflow translation if the fill kept an unresolvable
+        // overlap), and skews the spatial-focus queries built on this cache.
+        self.stable_snap_rects.insert(wl_surface.id(), frame);
+    }
+
+    pub fn unfill_window(&mut self, window: &Window) {
+        let Some(wl_surface) = window.wl_surface() else {
+            return;
+        };
+        let Some((saved_pos, saved_size)) = self.stage.take_fill_saved(window) else {
+            return;
+        };
+
+        // Visual center of the saved geometry, matching the convention unfit and
+        // the pending-recenter completion in `handlers/compositor.rs` use.
+        let bar = self.window_ssd_bar(window);
+        let target_center = Point::from((
+            saved_pos.x as f64 + saved_size.w as f64 / 2.0,
+            saved_pos.y as f64 - bar as f64 + (saved_size.h + bar) as f64 / 2.0,
+        ));
+
+        let pre_exit_size = window.geometry().size;
+        self.animate_window_geometry(window, saved_pos, saved_size);
+        self.send_size_configure(window, saved_size);
+
+        if saved_size == pre_exit_size {
+            // The exit configure re-sends the size the client already has, so no
+            // commit with a changed size will arrive to trigger the recenter —
+            // restore the position immediately instead.
+            self.map_window(window.clone(), saved_pos, false);
+            self.refresh_stable_snap_rect(window);
+        } else {
+            self.pending_recenter.insert(
+                wl_surface.id(),
+                PendingRecenter {
+                    target_center,
+                    pre_exit_size,
+                },
+            );
+        }
+    }
+
+    pub fn toggle_fill_window(&mut self, window: &Window) {
+        if self.stage.is_fill(window) {
+            self.unfill_window(window);
+        } else {
+            self.fill_window(window);
+        }
+    }
+
+    /// Send a plain sized configure — no Maximized/Fullscreen/Resizing state, so
+    /// the window resizes in place. Tiled stays set from map time, so clients
+    /// keep suppressing their own chrome, and the explicit size keeps SCTK from
+    /// reading "Tiled + None" as "hold current size".
+    fn send_size_configure(&self, window: &Window, size: Size<i32, Logical>) {
+        let Some(toplevel) = window.toplevel() else {
+            return;
+        };
+        toplevel.with_pending_state(|state| {
+            state.size = Some(size);
+        });
+        toplevel.send_configure();
+    }
+}
diff --git a/src/state/fit.rs b/src/state/fit.rs
index 9d16bb1..e020cda 100644
--- a/src/state/fit.rs
+++ b/src/state/fit.rs
@@ -5,7 +5,7 @@ use smithay::{
     wayland::seat::WaylandFocus,
 };
 
-use super::{DriftWm, PendingRecenter};
+use super::{DriftWm, PendingRecenter, ZoomAnimationAnchor};
 use driftwm::config;
 use driftwm::window_ext::WindowExt;
 
@@ -92,11 +92,15 @@ impl DriftWm {
             visual_center: center,
         } = self.compute_fit_geometry(window);
 
+        self.animate_window_geometry(window, new_loc, target_size);
         window.enter_fit_configure(target_size);
         self.map_window(window.clone(), new_loc, false);
         // After the map — set_fit needs the window's stage entry, which the
         // map guarantees even for a window that wasn't staged before.
         self.stage.set_fit(window, current_size);
+        // Fit translates the window and unfit restores by visual center, so a
+        // pre-fit fill's saved position would be permanently stale — drop it.
+        self.stage.clear_fill(window);
         // Don't refresh `stable_snap_rects` here — the fit canvas position
         // snap-touches nothing, so close-time `cluster_of` would degrade to
         // `{self}`. The pre-fit rect is the window's cluster identity.
@@ -106,9 +110,13 @@ impl DriftWm {
         let serial = smithay::utils::SERIAL_COUNTER.next_serial();
         self.raise_and_focus(window, serial);
         self.set_overview_return(None);
+        let viewport_center = self.usable_center_screen();
         self.with_output_state(|os| {
             os.momentum.stop();
-            os.zoom_animation_center = Some(center);
+            os.zoom_animation_anchor = Some(ZoomAnimationAnchor {
+                canvas: center,
+                screen: viewport_center,
+            });
             os.camera_target = Some(target_camera);
             os.zoom_target = Some(1.0);
         });
@@ -137,6 +145,7 @@ impl DriftWm {
         // then re-center using the real post-unfit size.
         let pre_exit_size = window.geometry().size;
 
+        self.animate_window_geometry(window, new_loc, saved_size);
         window.exit_fit_configure(saved_size);
         self.map_window(window.clone(), new_loc, false);
 
@@ -248,7 +257,20 @@ impl DriftWm {
                 .filter(|w| w != window)
                 .collect()
         };
+        let old_member_positions: Vec<_> = cluster_members
+            .iter()
+            .filter_map(|member| {
+                self.stage
+                    .position_of(member)
+                    .map(|loc| (member.clone(), loc))
+            })
+            .collect();
         self.shift_cluster_around_primary(window, old_rect, new_rect);
+        for (member, old_loc) in old_member_positions {
+            if let Some(new_loc) = self.stage.position_of(&member) {
+                self.animate_window_geometry_from(&member, old_loc, new_loc);
+            }
+        }
         self.fit_window(window);
         for member in &cluster_members {
             self.refresh_stable_snap_rect(member);
@@ -289,7 +311,20 @@ impl DriftWm {
                 .filter(|w| w != window)
                 .collect()
         };
+        let old_member_positions: Vec<_> = cluster_members
+            .iter()
+            .filter_map(|member| {
+                self.stage
+                    .position_of(member)
+                    .map(|loc| (member.clone(), loc))
+            })
+            .collect();
         self.shift_cluster_around_primary(window, old_rect, new_rect);
+        for (member, old_loc) in old_member_positions {
+            if let Some(new_loc) = self.stage.position_of(&member) {
+                self.animate_window_geometry_from(&member, old_loc, new_loc);
+            }
+        }
         self.unfit_window(window);
         for member in &cluster_members {
             self.refresh_stable_snap_rect(member);
diff --git a/src/state/fullscreen.rs b/src/state/fullscreen.rs
index 82c4f93..7983494 100644
--- a/src/state/fullscreen.rs
+++ b/src/state/fullscreen.rs
@@ -8,6 +8,17 @@ use super::{DriftWm, FocusTarget};
 use driftwm::window_ext::WindowExt;
 
 impl DriftWm {
+    /// Whether fullscreen occlusion should already hide the canvas underneath.
+    /// During entry the stage is logically fullscreen immediately, while the
+    /// visual transition keeps the previous scene visible until the window
+    /// reaches the output bounds.
+    pub(crate) fn is_output_visually_fullscreen(&self, output: &smithay::output::Output) -> bool {
+        self.is_output_fullscreen(output)
+            && self
+                .fullscreen_window_on(output)
+                .is_none_or(|window| !self.window_fullscreen_animation_active(&window))
+    }
+
     /// Resolve which output a window should fullscreen onto. An already-fullscreen
     /// window re-asserting with no requested output stays on its current output;
     /// otherwise a window-rule `output` wins, then the client-requested output,
@@ -122,6 +133,7 @@ impl DriftWm {
             let os = super::output_state(&output);
             (os.camera, os.zoom)
         };
+        let windowed_size = window.geometry().size;
 
         // A game that maps straight into fullscreen commits its first buffer at
         // a throwaway default before it learns it's fullscreen, and that size is
@@ -149,6 +161,30 @@ impl DriftWm {
 
         // Unpin into the fullscreen viewport; exit_fullscreen_on re-pins.
         let saved_pinned = self.stage.take_pin(window);
+        let camera_i32 = saved_camera.to_i32_round();
+        let (fullscreen_from_loc, fullscreen_from_size) = if let Some(site) = saved_pinned.as_ref()
+        {
+            (
+                Point::from((
+                    (camera_i32.x as f64 + site.screen_pos.x as f64).round() as i32,
+                    (camera_i32.y as f64 + site.screen_pos.y as f64).round() as i32,
+                )),
+                Size::from((windowed_size.w.max(1), windowed_size.h.max(1))),
+            )
+        } else {
+            (
+                Point::from((
+                    (camera_i32.x as f64 + (saved_location.x as f64 - saved_camera.x) * saved_zoom)
+                        .round() as i32,
+                    (camera_i32.y as f64 + (saved_location.y as f64 - saved_camera.y) * saved_zoom)
+                        .round() as i32,
+                )),
+                Size::from((
+                    (windowed_size.w as f64 * saved_zoom).round().max(1.0) as i32,
+                    (windowed_size.h as f64 * saved_zoom).round().max(1.0) as i32,
+                )),
+            )
+        };
 
         self.stage
             .set_fullscreen(&output.name(), window.clone(), saved_location, saved_size);
@@ -165,7 +201,7 @@ impl DriftWm {
             let mut os = super::output_state(&output);
             os.zoom = 1.0;
             os.zoom_target = None;
-            os.zoom_animation_center = None;
+            os.zoom_animation_anchor = None;
             os.camera_target = None;
             os.momentum.stop();
             os.overview_return = None;
@@ -177,12 +213,18 @@ impl DriftWm {
         // output's state directly: `set_camera` refuses to move a fullscreen
         // output (the window is pinned to its camera-origin), and this output's
         // stage fullscreen entry is already set above.
-        let camera_i32 = super::output_state(&output).camera.to_i32_round();
         super::output_state(&output).camera =
             Point::from((camera_i32.x as f64, camera_i32.y as f64));
 
         // Place window at viewport origin and raise
         self.map_window(window.clone(), camera_i32, true);
+        self.animate_window_fullscreen(
+            window,
+            fullscreen_from_loc,
+            fullscreen_from_size,
+            camera_i32,
+            viewport_size,
+        );
         self.raise_window(window, true);
         self.enforce_below_windows();
         self.update_output_from_camera();
@@ -272,6 +314,26 @@ impl DriftWm {
             return;
         };
 
+        // Capture the currently presented geometry before changing the stage.
+        // If entry is still animating, this is its intermediate visual rather
+        // than the fullscreen target, so reversing the transition cannot jump.
+        let parked_camera = super::output_state(output).camera;
+        let parked_zoom = super::output_state(output).zoom;
+        let parked_loc = self
+            .stage
+            .position_of(&entry.window)
+            .unwrap_or_else(|| parked_camera.to_i32_round());
+        let current_visual =
+            self.window_visual(&entry.window, parked_loc, entry.window.geometry().size);
+        let current_screen_loc: Point<f64, Logical> = Point::from((
+            (current_visual.loc.x - parked_camera.x) * parked_zoom,
+            (current_visual.loc.y - parked_camera.y) * parked_zoom,
+        ));
+        let current_screen_size: Size<f64, Logical> = Size::from((
+            current_visual.size.w * parked_zoom,
+            current_visual.size.h * parked_zoom,
+        ));
+
         entry.window.exit_fullscreen_configure(entry.saved_size);
 
         // Restore window position, camera, zoom on the specific output
@@ -290,6 +352,43 @@ impl DriftWm {
         if was_pinned {
             self.sync_pinned_locs();
         }
+
+        let target_loc = self
+            .stage
+            .position_of(&entry.window)
+            .unwrap_or(entry.saved_location);
+        let (from_loc, from_size): (Point<i32, Logical>, Size<i32, Logical>) =
+            if let Some(site) = self.stage.pin_of(&entry.window) {
+                // Pinned windows render directly in screen space. Express the
+                // current fullscreen visual relative to their restored screen site.
+                (
+                    Point::from((
+                        (target_loc.x as f64 + current_screen_loc.x - site.screen_pos.x as f64)
+                            .round() as i32,
+                        (target_loc.y as f64 + current_screen_loc.y - site.screen_pos.y as f64)
+                            .round() as i32,
+                    )),
+                    current_screen_size.to_i32_round(),
+                )
+            } else {
+                // Normal windows render through the restored camera/zoom. Convert
+                // the current screen rectangle back into that canvas coordinate
+                // system before animating toward the saved window geometry.
+                (
+                    Point::from((
+                        (ret.camera.x + current_screen_loc.x / ret.zoom).round() as i32,
+                        (ret.camera.y + current_screen_loc.y / ret.zoom).round() as i32,
+                    )),
+                    current_screen_size.downscale(ret.zoom).to_i32_round(),
+                )
+            };
+        self.animate_window_geometry_between(
+            &entry.window,
+            from_loc,
+            Size::from((from_size.w.max(1), from_size.h.max(1))),
+            target_loc,
+            entry.saved_size,
+        );
     }
 
     /// Restore an output's camera/zoom after fullscreen ends. Drops any
@@ -321,7 +420,7 @@ impl DriftWm {
             os.zoom = zoom;
             os.camera_target = None;
             os.zoom_target = None;
-            os.zoom_animation_center = None;
+            os.zoom_animation_anchor = None;
         }
         self.update_output_from_camera();
 
diff --git a/src/state/init.rs b/src/state/init.rs
index 2d39783..e73f6e9 100644
--- a/src/state/init.rs
+++ b/src/state/init.rs
@@ -287,6 +287,8 @@ impl DriftWm {
             pending_ssd: HashSet::new(),
             decoration_scale: 1,
             render: RenderCache::new(),
+            window_animations: Default::default(),
+            closing_snapshots: Vec::new(),
             dmabuf_state: DmabufState::new(),
             dmabuf_global: None,
             render_device: None,
diff --git a/src/state/mod.rs b/src/state/mod.rs
index 6b9293f..2bfb09e 100644
--- a/src/state/mod.rs
+++ b/src/state/mod.rs
@@ -2,6 +2,7 @@ mod animation;
 mod cluster_snapshot;
 mod cursor;
 mod errors;
+pub mod fill;
 pub mod fit;
 mod focus;
 mod fullscreen;
@@ -14,6 +15,7 @@ mod placement;
 mod reload;
 mod render_cache;
 mod viewport;
+mod window_animation;
 pub use cluster_snapshot::ClusterResizeSnapshot;
 pub use cursor::{CursorFrames, CursorState};
 pub use errors::ErrorSource;
@@ -247,12 +249,18 @@ pub enum ModeIntent {
 /// Per-output viewport state, stored on each `Output` via `UserDataMap`
 /// (wrapped in `Mutex` since `UserDataMap` requires `Sync`). !Send fields
 /// and non-Copy ownership types (fullscreen, lock_surface) stay on DriftWm.
+#[derive(Clone, Copy, Debug)]
+pub struct ZoomAnimationAnchor {
+    pub canvas: Point<f64, Logical>,
+    pub screen: Point<f64, Logical>,
+}
+
 #[derive(Clone)]
 pub struct OutputState {
     pub camera: Point<f64, Logical>,
     pub zoom: f64,
     pub zoom_target: Option<f64>,
-    pub zoom_animation_center: Option<Point<f64, Logical>>,
+    pub zoom_animation_anchor: Option<ZoomAnimationAnchor>,
     pub last_rendered_zoom: f64,
     pub overview_return: Option<(Point<f64, Logical>, f64)>,
     pub camera_target: Option<Point<f64, Logical>>,
@@ -284,7 +292,7 @@ pub fn init_output_state(
             camera,
             zoom: 1.0,
             zoom_target: None,
-            zoom_animation_center: None,
+            zoom_animation_anchor: None,
             last_rendered_zoom: f64::NAN,
             overview_return: None,
             camera_target: None,
@@ -439,6 +447,8 @@ pub struct DriftWm {
     /// (downscaling stays crisp; only upscaling blurs).
     pub decoration_scale: i32,
     pub render: RenderCache,
+    pub(crate) window_animations: window_animation::WindowAnimations,
+    pub(crate) closing_snapshots: Vec<crate::render::ClosingSnapshot>,
 
     pub dmabuf_state: DmabufState,
     pub dmabuf_global: Option<DmabufGlobal>,
@@ -763,6 +773,9 @@ impl DriftWm {
     /// half (camera restore) is NOT handled here — a caller unmapping a
     /// fullscreen window must tear that down first, as `toplevel_destroyed` does.
     pub fn unmap_window(&mut self, window: &Window) {
+        if let Some(surface) = window.wl_surface() {
+            self.window_animations.remove(&surface.id());
+        }
         self.stage.remove(window);
         membership::send_output_leaves(window);
     }
@@ -1314,6 +1327,15 @@ impl DriftWm {
             || os.momentum.velocity.y != 0.0
     }
 
+    /// Visual animations which are not owned by a single output.
+    ///
+    /// These currently include windows that may span outputs and closing
+    /// snapshots. Keeping this separate from timers such as key repeat makes
+    /// the render scheduler's "draw one final frame" rule explicit.
+    pub(crate) fn has_global_visual_animations(&self) -> bool {
+        self.window_animations.is_active() || !self.closing_snapshots.is_empty()
+    }
+
     /// True when `output_name`'s animated background is due for its next tick
     /// under `[background] animate_fps` (0 = every frame). The timestamp is
     /// stamped where the uniforms are actually pushed, in
@@ -1335,15 +1357,13 @@ impl DriftWm {
     }
 
     /// Outputs whose animated background can actually render: active, not
-    /// fullscreen, not DPMS-off. Fullscreen and DPMS-off outputs stop
-    /// rendering the background, so their `background_last_animate` stamps
-    /// go stale and would otherwise read as permanently due. Shared by the
-    /// idle due-check, the tick-timer arming wait, and the per-frame
-    /// dirty-marking so all three agree on which outputs count.
+    /// visually fullscreen, not DPMS-off. A fullscreen-entry transition keeps
+    /// its canvas visible until the window covers it, so its background remains
+    /// eligible for that short interval.
     pub(crate) fn background_render_eligible_outputs(&self) -> impl Iterator<Item = &Output> {
-        self.active_outputs
-            .iter()
-            .filter(|o| !self.is_output_fullscreen(o) && !self.dpms_off_outputs.contains(o))
+        self.active_outputs.iter().filter(|o| {
+            !self.is_output_visually_fullscreen(o) && !self.dpms_off_outputs.contains(o)
+        })
     }
 
     /// Owned-name variant of [`Self::background_render_eligible_outputs`] for
@@ -1372,6 +1392,7 @@ impl DriftWm {
             || self.cursor.exec_cursor_show_at.is_some()
             || self.cursor.exec_cursor_deadline.is_some()
             || self.cursor.is_animated()
+            || self.has_global_visual_animations()
     }
 
     pub fn flush_middle_click(&mut self, press_time: u32, release_time: Option<u32>) {
@@ -1813,13 +1834,15 @@ impl DriftWm {
     /// Viewport area minus layer-shell exclusive zones (panels, bars).
     pub fn get_usable_area(&self) -> Rectangle<i32, Logical> {
         self.active_output()
-            .map(|o| {
-                let map = smithay::desktop::layer_map_for_output(&o);
-                map.non_exclusive_zone()
-            })
+            .map(|o| self.usable_area_on(&o))
             .unwrap_or_else(|| Rectangle::new((0, 0).into(), (1, 1).into()))
     }
 
+    /// `output`'s usable area (viewport minus layer-shell exclusive zones).
+    pub fn usable_area_on(&self, output: &Output) -> Rectangle<i32, Logical> {
+        smithay::desktop::layer_map_for_output(output).non_exclusive_zone()
+    }
+
     /// Screen-space center of the usable area (= viewport center when no panels exist).
     pub fn usable_center_screen(&self) -> Point<f64, Logical> {
         self.active_output()
@@ -1965,6 +1988,7 @@ impl DriftWm {
             ("auto_anchor_snapshot", self.auto_anchor_snapshot.len()),
             ("pending_recenter", self.pending_recenter.len()),
             ("stable_snap_rects", self.stable_snap_rects.len()),
+            ("closing_snapshots", self.closing_snapshots.len()),
             (
                 "idle_inhibiting_surfaces",
                 self.idle_inhibiting_surfaces.len(),
@@ -2077,7 +2101,7 @@ mod tests {
             camera: Point::from(camera),
             zoom,
             zoom_target: None,
-            zoom_animation_center: None,
+            zoom_animation_anchor: None,
             last_rendered_zoom: zoom,
             overview_return: None,
             camera_target: None,
diff --git a/src/state/navigation.rs b/src/state/navigation.rs
index e97450a..b7d3928 100644
--- a/src/state/navigation.rs
+++ b/src/state/navigation.rs
@@ -14,7 +14,7 @@ use smithay::{
     wayland::seat::WaylandFocus,
 };
 
-use super::{DriftWm, PendingClickNavigate, output_state};
+use super::{DriftWm, PendingClickNavigate, ZoomAnimationAnchor, output_state};
 
 /// Max pointer travel (screen px) between press and release for a click to
 /// still count as a click rather than a drag. Beyond it, no auto-navigate — a
@@ -103,7 +103,10 @@ impl DriftWm {
         });
         let mut os = output_state(output);
         os.momentum.stop();
-        os.zoom_animation_center = Some(window_center);
+        os.zoom_animation_anchor = Some(ZoomAnimationAnchor {
+            canvas: window_center,
+            screen: vc,
+        });
         os.camera_target = Some(target);
         os.zoom_target = Some(target_zoom);
     }
diff --git a/src/state/viewport.rs b/src/state/viewport.rs
index 601e633..dab912a 100644
--- a/src/state/viewport.rs
+++ b/src/state/viewport.rs
@@ -56,13 +56,23 @@ impl DriftWm {
             output_state(&o).zoom_target = val;
         }
     }
-    pub fn zoom_animation_center(&self) -> Option<Point<f64, Logical>> {
+    pub fn zoom_animation_anchor(&self) -> Option<super::ZoomAnimationAnchor> {
         self.active_output()
-            .and_then(|o| output_state(&o).zoom_animation_center)
+            .and_then(|o| output_state(&o).zoom_animation_anchor)
     }
-    pub fn set_zoom_animation_center(&mut self, val: Option<Point<f64, Logical>>) {
+    pub fn set_zoom_animation_anchor(
+        &mut self,
+        canvas: Point<f64, Logical>,
+        screen: Point<f64, Logical>,
+    ) {
         if let Some(o) = self.active_output() {
-            output_state(&o).zoom_animation_center = val;
+            output_state(&o).zoom_animation_anchor =
+                Some(super::ZoomAnimationAnchor { canvas, screen });
+        }
+    }
+    pub fn clear_zoom_animation_anchor(&mut self) {
+        if let Some(o) = self.active_output() {
+            output_state(&o).zoom_animation_anchor = None;
         }
     }
     pub fn overview_return(&self) -> Option<(Point<f64, Logical>, f64)> {
diff --git a/src/state/window_animation.rs b/src/state/window_animation.rs
new file mode 100644
index 0000000..345dcd9
--- /dev/null
+++ b/src/state/window_animation.rs
@@ -0,0 +1,263 @@
+use std::collections::HashMap;
+use std::time::Duration;
+
+use smithay::desktop::Window;
+use smithay::reexports::wayland_server::{Resource, backend::ObjectId};
+use smithay::utils::{Logical, Point, Size};
+use smithay::wayland::seat::WaylandFocus;
+
+const OPEN_SCALE: f64 = 0.8;
+const DONE_EPSILON: f64 = 0.001;
+
+#[derive(Clone, Copy, Debug)]
+pub(crate) struct WindowVisual {
+    pub loc: Point<f64, Logical>,
+    pub size: Size<f64, Logical>,
+    pub alpha: f32,
+}
+
+#[derive(Clone, Copy, Debug)]
+enum GeometryRole {
+    Normal,
+    FullscreenEntry,
+}
+
+#[derive(Clone, Copy, Debug)]
+enum AnimationKind {
+    Open,
+    Geometry {
+        from_loc: Point<f64, Logical>,
+        from_size: Size<f64, Logical>,
+        to_loc: Point<f64, Logical>,
+        to_size: Size<f64, Logical>,
+        role: GeometryRole,
+    },
+}
+
+#[derive(Debug)]
+struct WindowAnimation {
+    window: Window,
+    kind: AnimationKind,
+    progress: f64,
+}
+
+#[derive(Default)]
+pub(crate) struct WindowAnimations {
+    animations: HashMap<ObjectId, WindowAnimation>,
+    pending_closes: HashMap<ObjectId, Window>,
+}
+
+impl WindowAnimations {
+    pub fn start_open(&mut self, window: &Window) {
+        let Some(surface) = window.wl_surface() else {
+            return;
+        };
+        self.animations.insert(
+            surface.id(),
+            WindowAnimation {
+                window: window.clone(),
+                kind: AnimationKind::Open,
+                progress: 0.0,
+            },
+        );
+    }
+
+    pub fn start_geometry(
+        &mut self,
+        window: &Window,
+        from_loc: Point<i32, Logical>,
+        from_size: Size<i32, Logical>,
+        to_loc: Point<i32, Logical>,
+        to_size: Size<i32, Logical>,
+    ) {
+        self.insert_geometry(
+            window,
+            from_loc,
+            from_size,
+            to_loc,
+            to_size,
+            GeometryRole::Normal,
+        );
+    }
+
+    pub fn start_fullscreen(
+        &mut self,
+        window: &Window,
+        from_loc: Point<i32, Logical>,
+        from_size: Size<i32, Logical>,
+        to_loc: Point<i32, Logical>,
+        to_size: Size<i32, Logical>,
+    ) {
+        self.insert_geometry(
+            window,
+            from_loc,
+            from_size,
+            to_loc,
+            to_size,
+            GeometryRole::FullscreenEntry,
+        );
+    }
+
+    fn insert_geometry(
+        &mut self,
+        window: &Window,
+        from_loc: Point<i32, Logical>,
+        from_size: Size<i32, Logical>,
+        to_loc: Point<i32, Logical>,
+        to_size: Size<i32, Logical>,
+        role: GeometryRole,
+    ) {
+        let Some(surface) = window.wl_surface() else {
+            return;
+        };
+        self.animations.insert(
+            surface.id(),
+            WindowAnimation {
+                window: window.clone(),
+                kind: AnimationKind::Geometry {
+                    from_loc: from_loc.to_f64(),
+                    from_size: from_size.to_f64(),
+                    to_loc: to_loc.to_f64(),
+                    to_size: to_size.to_f64(),
+                    role,
+                },
+                progress: 0.0,
+            },
+        );
+    }
+
+    pub fn is_fullscreen_transition(&self, window: &Window) -> bool {
+        window
+            .wl_surface()
+            .and_then(|surface| self.animations.get(&surface.id()))
+            .is_some_and(|animation| {
+                matches!(
+                    animation.kind,
+                    AnimationKind::Geometry {
+                        role: GeometryRole::FullscreenEntry,
+                        ..
+                    }
+                ) && animation.progress < 1.0
+            })
+    }
+
+    /// Queue a window for a one-shot GPU snapshot on the next rendered frame.
+    /// Returns false when the same close is already pending.
+    pub fn request_close(&mut self, window: &Window) -> bool {
+        let Some(surface) = window.wl_surface() else {
+            return false;
+        };
+        if self.pending_closes.contains_key(&surface.id()) {
+            return false;
+        }
+        self.pending_closes.insert(surface.id(), window.clone());
+        true
+    }
+
+    pub fn remove(&mut self, id: &ObjectId) {
+        self.animations.remove(id);
+        self.pending_closes.remove(id);
+    }
+
+    pub fn is_active(&self) -> bool {
+        self.animations
+            .values()
+            .any(|animation| animation.progress < 1.0)
+            || !self.pending_closes.is_empty()
+    }
+
+    pub fn close_pending(&self, id: &ObjectId) -> bool {
+        self.pending_closes.contains_key(id)
+    }
+
+    pub fn take_pending_close(&mut self, id: &ObjectId) -> Option<Window> {
+        self.pending_closes.remove(id)
+    }
+
+    pub fn tick(&mut self, dt: Duration, factor: f64) {
+        let frame_factor = 1.0 - (1.0 - factor).powf(dt.as_secs_f64() * 60.0);
+        self.animations.retain(|_, animation| {
+            if animation.progress < 1.0 {
+                animation.progress += (1.0 - animation.progress) * frame_factor;
+                if 1.0 - animation.progress <= DONE_EPSILON {
+                    animation.progress = 1.0;
+                }
+            }
+
+            match animation.kind {
+                AnimationKind::Open => animation.progress < 1.0,
+                AnimationKind::Geometry { to_size, .. } => {
+                    // A configure is asynchronous. Keep the endpoint transform
+                    // without scheduling frames until the client commits the
+                    // requested size; otherwise a slow client briefly snaps
+                    // back to its old buffer when the timed animation ends.
+                    animation.progress < 1.0
+                        || animation.window.geometry().size != to_size.to_i32_round()
+                }
+            }
+        });
+    }
+
+    pub fn visual(
+        &self,
+        id: &ObjectId,
+        target_loc: Point<i32, Logical>,
+        target_size: Size<i32, Logical>,
+    ) -> WindowVisual {
+        let target_loc = target_loc.to_f64();
+        let target_size = target_size.to_f64();
+        let Some(animation) = self.animations.get(id) else {
+            return WindowVisual {
+                loc: target_loc,
+                size: target_size,
+                alpha: 1.0,
+            };
+        };
+        let p = animation.progress.clamp(0.0, 1.0);
+        match animation.kind {
+            AnimationKind::Open => {
+                let scale = OPEN_SCALE + (1.0 - OPEN_SCALE) * p;
+                WindowVisual {
+                    loc: target_loc
+                        + (target_size.to_point() - target_size.to_point().upscale(scale))
+                            .downscale(2.0),
+                    size: target_size.upscale(scale),
+                    alpha: p as f32,
+                }
+            }
+            AnimationKind::Geometry {
+                from_loc,
+                from_size,
+                to_loc,
+                to_size,
+                ..
+            } => WindowVisual {
+                loc: lerp_point(from_loc, to_loc, p),
+                size: lerp_size(from_size, to_size, p),
+                alpha: 1.0,
+            },
+        }
+    }
+}
+
+fn lerp_point(
+    from: Point<f64, Logical>,
+    to: Point<f64, Logical>,
+    progress: f64,
+) -> Point<f64, Logical> {
+    Point::from((
+        from.x + (to.x - from.x) * progress,
+        from.y + (to.y - from.y) * progress,
+    ))
+}
+
+fn lerp_size(
+    from: Size<f64, Logical>,
+    to: Size<f64, Logical>,
+    progress: f64,
+) -> Size<f64, Logical> {
+    Size::from((
+        from.w + (to.w - from.w) * progress,
+        from.h + (to.h - from.h) * progress,
+    ))
+}
diff --git a/src/tests/configure_sequences.rs b/src/tests/configure_sequences.rs
index 9699958..77da0e9 100644
--- a/src/tests/configure_sequences.rs
+++ b/src/tests/configure_sequences.rs
@@ -1,6 +1,10 @@
 //! Exact configure sequences as the client sees them — the desync class where
 //! a toolkit acks one configure while the compositor already believes another.
 
+use driftwm::config::{Action, Config, DecorationMode};
+use smithay::reexports::wayland_server::Resource;
+use smithay::utils::Point;
+
 use super::{Fixture, window_by_app_id};
 
 /// Map one toplevel with a buffer at `size`, settle, and drain the configure
@@ -148,3 +152,356 @@ fn fit_round_trip_restores_exact_size() {
     );
     assert!(!f.state().stage.is_fit(&window));
 }
+
+#[test]
+fn fill_grows_to_usable_minus_gap() {
+    let mut f = Fixture::new();
+    f.add_output(1, (1920, 1080));
+    let id = f.add_client();
+
+    // Even size to sidestep the known 1px truncation quirk.
+    let surface = map_settled(&mut f, id, "fill", (800, 600));
+    let window = window_by_app_id(&mut f, "fill").unwrap();
+
+    f.state().toggle_fill_window(&window);
+    f.double_roundtrip(id);
+
+    // Usable 1920×1080 minus a 12px gap on every side, no SSD bar / border on a
+    // default CSD window → the content fills 1896×1056.
+    let configures = f.client(id).window(&surface).format_recent_configures();
+    assert!(
+        configures.contains("size: 1896 × 1056"),
+        "fill must configure the free-space size, got:\n{configures}"
+    );
+    assert!(f.state().stage.is_fill(&window));
+}
+
+#[test]
+fn fill_round_trip_restores_size_and_position() {
+    let mut f = Fixture::new();
+    f.add_output(1, (1920, 1080));
+    let id = f.add_client();
+
+    let surface = map_settled(&mut f, id, "fill", (800, 600));
+    let window = window_by_app_id(&mut f, "fill").unwrap();
+    let pre_pos = f.state().stage.position_of(&window).unwrap();
+    let pre_size = window.geometry().size;
+
+    // Fill, then let the client adopt the filled size as a real client would.
+    f.state().toggle_fill_window(&window);
+    f.double_roundtrip(id);
+    let cw = f.client(id).window(&surface);
+    let (w, h) = cw.configures_received.last().unwrap().1.size;
+    cw.set_size(w as u16, h as u16);
+    cw.ack_last_and_commit();
+    f.double_roundtrip(id);
+    f.client(id).window(&surface).format_recent_configures();
+    assert!(f.state().stage.is_fill(&window));
+
+    // Unfill: the exit configure restores the exact pre-fill size, and once the
+    // client commits it the pending recenter restores the pre-fill position.
+    f.state().toggle_fill_window(&window);
+    f.double_roundtrip(id);
+    let configures = f.client(id).window(&surface).format_recent_configures();
+    assert!(
+        configures.contains(&format!("size: {} × {}", pre_size.w, pre_size.h)),
+        "unfill must restore the exact pre-fill size, got:\n{configures}"
+    );
+    let cw = f.client(id).window(&surface);
+    let (w, h) = cw.configures_received.last().unwrap().1.size;
+    cw.set_size(w as u16, h as u16);
+    cw.ack_last_and_commit();
+    f.double_roundtrip(id);
+
+    assert!(!f.state().stage.is_fill(&window));
+    assert_eq!(
+        f.state().stage.position_of(&window),
+        Some(pre_pos),
+        "unfill must restore the exact pre-fill position"
+    );
+}
+
+#[test]
+fn fill_stops_at_neighbor() {
+    let mut f = Fixture::new();
+    f.add_output(1, (1920, 1080));
+    let id = f.add_client();
+
+    let a_surface = map_settled(&mut f, id, "a", (800, 600));
+    let _b_surface = map_settled(&mut f, id, "b", (400, 1056));
+    let a = window_by_app_id(&mut f, "a").unwrap();
+    let b = window_by_app_id(&mut f, "b").unwrap();
+
+    // Park B to A's right, spanning the usable height, so it caps A's rightward
+    // growth regardless of the axis order fill picks.
+    f.state()
+        .map_window(b.clone(), Point::from((500, -528)), false);
+    let b_loc = f.state().stage.position_of(&b).unwrap();
+
+    f.state().toggle_fill_window(&a);
+    f.double_roundtrip(id);
+
+    let gap = f.state().config.snap_gap as i32;
+    let a_loc = f.state().stage.position_of(&a).unwrap();
+    let (w, _h) = f
+        .client(id)
+        .window(&a_surface)
+        .configures_received
+        .last()
+        .unwrap()
+        .1
+        .size;
+    // A's right content edge stops exactly a gap short of B's left edge.
+    assert_eq!(a_loc.x + w, b_loc.x - gap);
+    assert!(f.state().stage.is_fill(&a));
+}
+
+#[test]
+fn fill_shrinks_out_of_overlap_with_neighbor() {
+    let mut f = Fixture::new();
+    f.add_output(1, (1920, 1080));
+    let id = f.add_client();
+
+    let a_surface = map_settled(&mut f, id, "a", (800, 600));
+    let _b_surface = map_settled(&mut f, id, "b", (400, 1056));
+    let a = window_by_app_id(&mut f, "a").unwrap();
+    let b = window_by_app_id(&mut f, "b").unwrap();
+
+    // Park B spanning the usable height, then drop A so it overlaps B's left
+    // portion. Fill must pull A's right edge back out of B before growing the
+    // free sides — the shrink phase, not just growth stopping short.
+    f.state()
+        .map_window(b.clone(), Point::from((500, -528)), false);
+    f.state()
+        .map_window(a.clone(), Point::from((300, 0)), false);
+    let b_loc = f.state().stage.position_of(&b).unwrap();
+
+    f.state().toggle_fill_window(&a);
+    f.double_roundtrip(id);
+
+    let gap = f.state().config.snap_gap as i32;
+    let a_loc = f.state().stage.position_of(&a).unwrap();
+    let (w, _h) = f
+        .client(id)
+        .window(&a_surface)
+        .configures_received
+        .last()
+        .unwrap()
+        .1
+        .size;
+    // A's right content edge ends exactly a gap short of B's left edge, even
+    // though A started overlapping B.
+    assert_eq!(a_loc.x + w, b_loc.x - gap);
+    assert!(f.state().stage.is_fill(&a));
+}
+
+#[test]
+fn fill_on_fit_window_is_noop() {
+    let mut f = Fixture::new();
+    f.add_output(1, (1920, 1080));
+    let id = f.add_client();
+
+    let _surface = map_settled(&mut f, id, "fit", (800, 600));
+    let window = window_by_app_id(&mut f, "fit").unwrap();
+
+    f.state().toggle_fit_window(&window);
+    assert!(f.state().stage.is_fit(&window));
+
+    // A maximized-by-fit window is fit's business; fill leaves it untouched.
+    f.state().fill_window(&window);
+    assert!(!f.state().stage.is_fill(&window));
+    assert!(f.state().stage.is_fit(&window));
+}
+
+#[test]
+fn fill_on_pinned_window_is_noop() {
+    let mut f = Fixture::new();
+    f.add_output(1, (1920, 1080));
+    let id = f.add_client();
+
+    let _surface = map_settled(&mut f, id, "pin", (800, 600));
+    let window = window_by_app_id(&mut f, "pin").unwrap();
+
+    f.state().execute_action(&Action::TogglePinToScreen);
+    assert!(
+        f.state().is_pinned(&window),
+        "precondition: window is pinned"
+    );
+
+    // The action's is_canvas_window filter drops pinned windows before toggle.
+    f.state().execute_action(&Action::FillWindow);
+    assert!(!f.state().stage.is_fill(&window));
+}
+
+#[test]
+fn fill_already_filling_does_not_set_membership() {
+    let mut f = Fixture::new();
+    f.add_output(1, (1920, 1080));
+    let id = f.add_client();
+
+    let surface = map_settled(&mut f, id, "fill", (800, 600));
+    let window = window_by_app_id(&mut f, "fill").unwrap();
+
+    // Fill and adopt the filled geometry.
+    f.state().toggle_fill_window(&window);
+    f.double_roundtrip(id);
+    let cw = f.client(id).window(&surface);
+    let (w, h) = cw.configures_received.last().unwrap().1.size;
+    cw.set_size(w as u16, h as u16);
+    cw.ack_last_and_commit();
+    f.double_roundtrip(id);
+
+    // Drop the restore point (as a manual resize/move would), then fill again:
+    // the window already fills its free space, so the geometry is a no-op and no
+    // fill membership is recorded.
+    f.state().stage.clear_fill(&window);
+    f.state().fill_window(&window);
+    assert!(!f.state().stage.is_fill(&window));
+}
+
+#[test]
+fn fill_at_zoom_and_pan_uses_canvas_space_usable_area() {
+    let mut f = Fixture::new();
+    f.add_output(1, (1920, 1080));
+    let id = f.add_client();
+
+    let surface = map_settled(&mut f, id, "fill", (800, 600));
+    let window = window_by_app_id(&mut f, "fill").unwrap();
+
+    // Zoom to 0.5 and pan the camera: the usable area is a screen rect, so the
+    // free canvas region fill grows into is (screen / zoom). Even numbers and
+    // zoom 0.5 keep the screen→canvas conversion exact, dodging the 1px quirk.
+    f.state().set_zoom(0.5);
+    f.state().set_camera(Point::from((5000.0, 5000.0)));
+    // Park the window inside the panned viewport so it intersects the bounds.
+    f.state()
+        .map_window(window.clone(), Point::from((6000, 6000)), false);
+
+    f.state().toggle_fill_window(&window);
+    f.double_roundtrip(id);
+
+    // Canvas bounds = camera + screen/zoom = [5000,8840]×[5000,7160]; inset by a
+    // 12px gap → free region 3816 × 2136 at canvas top-left (5012, 5012).
+    let configures = f.client(id).window(&surface).format_recent_configures();
+    assert!(
+        configures.contains("size: 3816 × 2136"),
+        "fill must configure the canvas-space free size, got:\n{configures}"
+    );
+    assert_eq!(
+        f.state().stage.position_of(&window),
+        Some(Point::from((5012, 5012))),
+        "fill must map the window at the gap-inset canvas top-left"
+    );
+    assert!(f.state().stage.is_fill(&window));
+}
+
+fn config_ssd() -> Config {
+    let mut config = Config::default();
+    config.decorations.default_mode = DecorationMode::Server;
+    config.decorations.border_width = 5;
+    config
+}
+
+#[test]
+fn fill_on_ssd_window_round_trips_bar_and_border() {
+    let mut f = Fixture::with_config(config_ssd());
+    f.add_output(1, (1920, 1080));
+    let id = f.add_client();
+
+    let surface = map_settled(&mut f, id, "fill", (800, 600));
+    let window = window_by_app_id(&mut f, "fill").unwrap();
+    // Precondition: the window actually carries a server-side title bar.
+    assert_eq!(f.state().window_ssd_bar(&window), 25);
+
+    // Pin the camera (mapping a window pans it to center) so the filled canvas
+    // position is deterministic; keep the window inside the viewport.
+    f.state().set_camera(Point::from((0.0, 0.0)));
+    f.state()
+        .map_window(window.clone(), Point::from((400, 300)), false);
+
+    f.state().toggle_fill_window(&window);
+    f.double_roundtrip(id);
+
+    // The free frame region is 1896 × 1056 (usable minus a 12px gap). The client
+    // content size is that minus a 5px border per side, and on height also the
+    // 25px title bar: 1886 × 1021 — proving the chrome inflation round-trips.
+    let configures = f.client(id).window(&surface).format_recent_configures();
+    assert!(
+        configures.contains("size: 1886 × 1021"),
+        "fill must deflate the frame by border and bar, got:\n{configures}"
+    );
+    assert_eq!(
+        f.state().stage.position_of(&window),
+        Some(Point::from((17, 42))),
+        "fill loc must offset the content by border and bar"
+    );
+    assert!(f.state().stage.is_fill(&window));
+}
+
+/// Fill must record its rect as the window's settled footprint. Leaving the
+/// pre-fill rect cached makes every later commit read as "grew past settled" —
+/// a perpetual reflow scan once the fill state is cleared (move-grab start,
+/// nudge), and a real translation whenever the fill kept an unresolvable
+/// overlap. A commit after the clear must leave the window in place.
+#[test]
+fn fill_records_settled_footprint() {
+    let mut f = Fixture::new();
+    f.add_output(1, (1920, 1080));
+    let id = f.add_client();
+
+    let a_surface = map_settled(&mut f, id, "a", (800, 600));
+    let _b_surface = map_settled(&mut f, id, "b", (400, 1056));
+    let a = window_by_app_id(&mut f, "a").unwrap();
+    let b = window_by_app_id(&mut f, "b").unwrap();
+
+    // Pin the camera (mapping pans it) and park A settled and gap-adjacent to
+    // B: the settled adjacency is the reflow's anchor precondition.
+    let gap = f.state().config.snap_gap as i32;
+    f.state().set_camera(Point::from((0.0, 0.0)));
+    f.state()
+        .map_window(a.clone(), Point::from((400, 300)), false);
+    f.state().refresh_stable_snap_rect(&a);
+    f.state()
+        .map_window(b.clone(), Point::from((1200 + gap, 300)), false);
+
+    f.state().toggle_fill_window(&a);
+    assert!(f.state().stage.is_fill(&a), "fill must not silently no-op");
+    f.double_roundtrip(id);
+    let (w, h) = f
+        .client(id)
+        .window(&a_surface)
+        .configures_received
+        .last()
+        .unwrap()
+        .1
+        .size;
+    let win = f.client(id).window(&a_surface);
+    win.set_size(w as u16, h as u16);
+    win.attach_new_buffer();
+    win.ack_last_and_commit();
+    f.double_roundtrip(id);
+    let filled_loc = f.state().stage.position_of(&a).unwrap();
+
+    // The settled footprint is the filled frame, not the stale pre-fill rect.
+    let a_id = super::server_surface(&a).id();
+    let stable = f.state().stable_snap_rects.get(&a_id).copied().unwrap();
+    assert_eq!(
+        (stable.x_low, stable.y_low, stable.x_high, stable.y_high),
+        (12.0, 12.0, 1200.0, 1068.0),
+        "fill must cache its target rect as the settled footprint"
+    );
+
+    // Re-anchor: every move path (grab start, nudge, send-to-output) funnels
+    // through clear_fill, then the app redraws before any grab-end settle.
+    f.state().stage.clear_fill(&a);
+    let win = f.client(id).window(&a_surface);
+    win.attach_new_buffer();
+    win.commit();
+    f.double_roundtrip(id);
+
+    assert_eq!(
+        f.state().stage.position_of(&a),
+        Some(filled_loc),
+        "a redraw commit after clear_fill must not translate the filled window"
+    );
+}
```diff
