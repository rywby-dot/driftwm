use std::collections::HashMap;
use std::time::Duration;

use smithay::desktop::Window;
use smithay::reexports::wayland_server::{Resource, backend::ObjectId};
use smithay::utils::{Logical, Point, Size};
use smithay::wayland::seat::WaylandFocus;

const OPEN_SCALE: f64 = 0.8;
const DONE_EPSILON: f64 = 0.001;

#[derive(Clone, Copy, Debug)]
pub(crate) struct WindowVisual {
    pub loc: Point<f64, Logical>,
    pub size: Size<f64, Logical>,
    pub alpha: f32,
}

#[derive(Clone, Copy, Debug)]
enum AnimationKind {
    Open,
    Geometry {
        from_loc: Point<f64, Logical>,
        from_size: Size<f64, Logical>,
        to_loc: Point<f64, Logical>,
        to_size: Size<f64, Logical>,
    },
}

#[derive(Debug)]
struct WindowAnimation {
    kind: AnimationKind,
    progress: f64,
}

#[derive(Default)]
pub(crate) struct WindowAnimations {
    animations: HashMap<ObjectId, WindowAnimation>,
    pending_closes: HashMap<ObjectId, Window>,
}

impl WindowAnimations {
    pub fn start_open(&mut self, window: &Window) {
        let Some(surface) = window.wl_surface() else {
            return;
        };
        self.animations.insert(
            surface.id(),
            WindowAnimation {
                kind: AnimationKind::Open,
                progress: 0.0,
            },
        );
    }

    pub fn start_geometry(
        &mut self,
        window: &Window,
        from_loc: Point<i32, Logical>,
        from_size: Size<i32, Logical>,
        to_loc: Point<i32, Logical>,
        to_size: Size<i32, Logical>,
    ) {
        let Some(surface) = window.wl_surface() else {
            return;
        };
        self.animations.insert(
            surface.id(),
            WindowAnimation {
                kind: AnimationKind::Geometry {
                    from_loc: from_loc.to_f64(),
                    from_size: from_size.to_f64(),
                    to_loc: to_loc.to_f64(),
                    to_size: to_size.to_f64(),
                },
                progress: 0.0,
            },
        );
    }

    /// Queue a window for a one-shot GPU snapshot on the next rendered frame.
    /// Returns false when the same close is already pending.
    pub fn request_close(&mut self, window: &Window) -> bool {
        let Some(surface) = window.wl_surface() else {
            return false;
        };
        if self.pending_closes.contains_key(&surface.id()) {
            return false;
        }
        self.pending_closes.insert(surface.id(), window.clone());
        true
    }

    pub fn remove(&mut self, id: &ObjectId) {
        self.animations.remove(id);
        self.pending_closes.remove(id);
    }

    pub fn is_active(&self) -> bool {
        !self.animations.is_empty() || !self.pending_closes.is_empty()
    }

    pub fn close_pending(&self, id: &ObjectId) -> bool {
        self.pending_closes.contains_key(id)
    }

    pub fn take_pending_close(&mut self, id: &ObjectId) -> Option<Window> {
        self.pending_closes.remove(id)
    }

    pub fn tick(&mut self, dt: Duration, factor: f64) {
        let frame_factor = 1.0 - (1.0 - factor).powf(dt.as_secs_f64() * 60.0);
        self.animations.retain(|_, animation| {
            animation.progress += (1.0 - animation.progress) * frame_factor;
            1.0 - animation.progress > DONE_EPSILON
        });
    }

    pub fn visual(
        &self,
        id: &ObjectId,
        target_loc: Point<i32, Logical>,
        target_size: Size<i32, Logical>,
    ) -> WindowVisual {
        let target_loc = target_loc.to_f64();
        let target_size = target_size.to_f64();
        let Some(animation) = self.animations.get(id) else {
            return WindowVisual {
                loc: target_loc,
                size: target_size,
                alpha: 1.0,
            };
        };
        let p = animation.progress.clamp(0.0, 1.0);
        match animation.kind {
            AnimationKind::Open => {
                let scale = OPEN_SCALE + (1.0 - OPEN_SCALE) * p;
                WindowVisual {
                    loc: target_loc
                        + (target_size.to_point() - target_size.to_point().upscale(scale))
                            .downscale(2.0),
                    size: target_size.upscale(scale),
                    alpha: p as f32,
                }
            }
            AnimationKind::Geometry {
                from_loc,
                from_size,
                to_loc,
                to_size,
            } => WindowVisual {
                loc: lerp_point(from_loc, to_loc, p),
                size: lerp_size(from_size, to_size, p),
                alpha: 1.0,
            },
        }
    }
}

fn lerp_point(
    from: Point<f64, Logical>,
    to: Point<f64, Logical>,
    progress: f64,
) -> Point<f64, Logical> {
    Point::from((
        from.x + (to.x - from.x) * progress,
        from.y + (to.y - from.y) * progress,
    ))
}

fn lerp_size(
    from: Size<f64, Logical>,
    to: Size<f64, Logical>,
    progress: f64,
) -> Size<f64, Logical> {
    Size::from((
        from.w + (to.w - from.w) * progress,
        from.h + (to.h - from.h) * progress,
    ))
}
