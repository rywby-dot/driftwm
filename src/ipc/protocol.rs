//! Shared IPC protocol types for the compositor and the `driftwm msg` client.
//!
//! The transport is line-delimited JSON over a Unix socket: one `Request` per
//! line, one `Reply` per line. Keeping it JSON means the socket is debuggable
//! with `socat` and usable from any scripting language, not just `driftwm msg`.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Selects a window: by stable id (JSON number) or case-insensitive `app_id`
/// substring (JSON string). Untagged, so the wire form is just `5` or `"term"`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WindowSelector {
    Id(u64),
    AppId(String),
}

/// A command from a client to the compositor. Variants carrying `Option<_>` read
/// when `None` and write when `Some`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Request {
    /// Coordinates are the viewport center, Y-up.
    Camera(Option<(f64, f64)>),
    /// Set value is clamped to the supported range (out to fit-all, in to native
    /// — no magnification).
    Zoom(Option<f64>),
    /// `short` reports the configured `input.keyboard.layout` code for the active
    /// group (e.g. `ru`) instead of the full XKB display name (e.g. `Russian`).
    Layout {
        short: bool,
    },
    State,
    /// Focus a window when `Some` (by [`WindowSelector`]); read the focused
    /// window's `app_id` when `None`.
    Focus(Option<WindowSelector>),
    /// Move or query a window. `window` `None` targets the focused window; `to`
    /// `None` reads the position instead of setting it. Coordinates are
    /// window-center, Y-up (the window-rule convention).
    Move {
        #[serde(default)]
        window: Option<WindowSelector>,
        #[serde(default)]
        to: Option<(i32, i32)>,
    },
    /// Close a window (the focused one when `None`); errors when nothing matches.
    Close(Option<WindowSelector>),
    /// Run a config action by its config-grammar string, e.g. `"switch-layout
    /// next"`. Any keybindable action is reachable, so one-shot ops live here
    /// rather than as their own commands.
    Action(String),
    /// Capture to a PNG at `path` (absolute), at `scale` pixels per canvas unit.
    /// Windows render with full chrome; `region`/`all` include the background, a
    /// `window` capture stays transparent (see [`ScreenshotTarget`]).
    Screenshot {
        target: ScreenshotTarget,
        scale: f64,
        path: String,
    },
}

/// What a [`Request::Screenshot`] captures.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScreenshotTarget {
    /// The active output's current viewport on the canvas — what's visible there
    /// (the default for a bare `screenshot`). Panels/layer-shells are excluded.
    Viewport,
    /// A single window (the focused one when `window` is `None`), isolated on
    /// transparency.
    Window {
        #[serde(default)]
        window: Option<WindowSelector>,
    },
    /// The bounding box of all non-widget windows.
    All,
    /// An explicit rectangle. Canvas coords are center/Y-up (the window-rule
    /// convention); with `from_screen`, `(x, y, w, h)` is an output-screen pixel
    /// rect (e.g. from `slurp`) mapped to the canvas via the active viewport.
    Region {
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        from_screen: bool,
    },
}

/// A successful reply payload. Pairs with [`Request`] variants.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Response {
    Camera {
        x: f64,
        y: f64,
    },
    Zoom(f64),
    Layout(String),
    State {
        camera: (f64, f64),
        zoom: f64,
        windows: Vec<WindowInfo>,
        /// Fullscreen + screen-pinned windows, which live in screen space and
        /// are therefore excluded from `windows` (canvas coords). Empty when none.
        #[serde(default)]
        fullscreen: Vec<OutputFullscreen>,
        #[serde(default)]
        pinned: Vec<OutputPinned>,
        /// Namespaces of screen-space layer-shell surfaces (bars, OSKs,
        /// overlays) — the `app_id` a window rule matches them by.
        #[serde(default)]
        layers: Vec<String>,
        #[serde(default)]
        canvas_layers: Vec<CanvasLayerInfo>,
    },
    Focused(Option<String>),
    /// Window-center, Y-up coordinates.
    Position {
        x: i32,
        y: i32,
    },
    /// A written screenshot: absolute `path` and pixel dimensions.
    Screenshot {
        path: String,
        width: u32,
        height: u32,
    },
    Ok,
}

/// The result of a request: `Ok(Response)` or a human-readable error string.
pub type Reply = Result<Response, String>;

/// One window in the canvas inventory (`position` = window center, Y-up).
///
/// Shared by the IPC [`Response::State`] payload and the
/// `$XDG_RUNTIME_DIR/driftwm/state` file so the two representations can't drift.
/// `id` is the compositor-session-stable window handle, usable as
/// [`WindowSelector::Id`] to target this window.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowInfo {
    pub id: u64,
    pub app_id: String,
    pub title: String,
    pub position: [i32; 2],
    pub size: [i32; 2],
    pub is_focused: bool,
    pub is_widget: bool,
}

/// A fullscreen window in the IPC `state` reply — one per fullscreened output.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputFullscreen {
    pub id: u64,
    pub output: String,
    pub app_id: String,
    pub title: String,
}

/// A screen-pinned window in the IPC `state` reply. `position` is the
/// output-relative top-left in screen pixels (Y-down); `size` in pixels.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputPinned {
    pub id: u64,
    pub output: String,
    pub app_id: String,
    pub title: String,
    pub position: [i32; 2],
    pub size: [i32; 2],
}

/// A canvas-positioned layer surface, shared by the IPC `state` reply and the
/// state file's `canvas_layers=` line. `app_id` is the layer-shell namespace;
/// `position` uses rule coordinates (Y-up, window-centered), like
/// [`WindowInfo`]. The top-left anchor is frozen at map time while the center
/// is derived from the *current* size, so for a surface that grew after
/// mapping the reported center drifts from the rule that placed it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanvasLayerInfo {
    pub app_id: String,
    pub position: [i32; 2],
    pub size: [i32; 2],
}

/// Path to the IPC socket for a given `WAYLAND_DISPLAY` name:
/// `$XDG_RUNTIME_DIR/driftwm/ipc-<wayland_display>.sock` (falls back to `/tmp`).
///
/// Deriving the name from the wayland display lets each compositor instance own
/// a distinct socket and lets `driftwm msg` auto-target the session it runs in.
pub fn socket_path(wayland_display: &str) -> PathBuf {
    let dir = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".to_string());
    PathBuf::from(dir)
        .join("driftwm")
        .join(format!("ipc-{wayland_display}.sock"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roundtrip<T>(value: &T)
    where
        T: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug,
    {
        let json = serde_json::to_string(value).unwrap();
        let back: T = serde_json::from_str(&json).unwrap();
        assert_eq!(value, &back);
    }

    #[test]
    fn request_roundtrip() {
        for r in [
            Request::Camera(None),
            Request::Camera(Some((100.0, -200.0))),
            Request::Zoom(Some(2.0)),
            Request::Layout { short: false },
            Request::Layout { short: true },
            Request::State,
            Request::Focus(None),
            Request::Focus(Some(WindowSelector::AppId("alacritty".into()))),
            Request::Focus(Some(WindowSelector::Id(5))),
            Request::Move {
                window: None,
                to: None,
            },
            Request::Move {
                window: None,
                to: Some((0, 0)),
            },
            Request::Move {
                window: Some(WindowSelector::Id(3)),
                to: None,
            },
            Request::Move {
                window: Some(WindowSelector::AppId("foot".into())),
                to: Some((100, 200)),
            },
            Request::Close(None),
            Request::Close(Some(WindowSelector::Id(7))),
            Request::Action("switch-layout next".into()),
            Request::Screenshot {
                target: ScreenshotTarget::Viewport,
                scale: 1.0,
                path: "/tmp/view.png".into(),
            },
            Request::Screenshot {
                target: ScreenshotTarget::Window { window: None },
                scale: 2.0,
                path: "/tmp/shot.png".into(),
            },
            Request::Screenshot {
                target: ScreenshotTarget::Window {
                    window: Some(WindowSelector::Id(2)),
                },
                scale: 2.0,
                path: "/tmp/shot.png".into(),
            },
            Request::Screenshot {
                target: ScreenshotTarget::Region {
                    x: -100,
                    y: 200,
                    w: 640,
                    h: 480,
                    from_screen: true,
                },
                scale: 1.0,
                path: "/tmp/region.png".into(),
            },
        ] {
            roundtrip(&r);
        }
    }

    #[test]
    fn selector_wire_forms_parse() {
        // Untagged: a bare string is an app_id, a bare number an id, null none.
        assert_eq!(
            serde_json::from_str::<Request>(r#"{"Focus":"term"}"#).unwrap(),
            Request::Focus(Some(WindowSelector::AppId("term".into())))
        );
        assert_eq!(
            serde_json::from_str::<Request>(r#"{"Focus":5}"#).unwrap(),
            Request::Focus(Some(WindowSelector::Id(5)))
        );
        assert_eq!(
            serde_json::from_str::<Request>(r#"{"Focus":null}"#).unwrap(),
            Request::Focus(None)
        );
        // Move fields both default when omitted.
        assert_eq!(
            serde_json::from_str::<Request>(r#"{"Move":{}}"#).unwrap(),
            Request::Move {
                window: None,
                to: None
            }
        );
        // Screenshot window target defaults its selector too.
        assert_eq!(
            serde_json::from_str::<ScreenshotTarget>(r#"{"Window":{}}"#).unwrap(),
            ScreenshotTarget::Window { window: None }
        );
    }

    #[test]
    fn reply_roundtrip() {
        let windows = vec![WindowInfo {
            id: 1,
            app_id: "foo".into(),
            title: "bar".into(),
            position: [10, -20],
            size: [640, 480],
            is_focused: true,
            is_widget: false,
        }];
        let replies: Vec<Reply> = vec![
            Ok(Response::Camera { x: 1.0, y: 2.0 }),
            Ok(Response::Zoom(1.5)),
            Ok(Response::State {
                camera: (0.0, 0.0),
                zoom: 1.0,
                windows,
                fullscreen: vec![OutputFullscreen {
                    id: 2,
                    output: "DP-1".into(),
                    app_id: "mpv".into(),
                    title: "video".into(),
                }],
                pinned: vec![OutputPinned {
                    id: 3,
                    output: "HDMI-A-1".into(),
                    app_id: "pavucontrol".into(),
                    title: "Volume".into(),
                    position: [20, 40],
                    size: [320, 240],
                }],
                layers: vec!["waybar".into()],
                canvas_layers: vec![CanvasLayerInfo {
                    app_id: "drift-clock".into(),
                    position: [0, 200],
                    size: [311, 136],
                }],
            }),
            Ok(Response::Focused(None)),
            Ok(Response::Ok),
            Err("no focused window".into()),
        ];
        for reply in &replies {
            let json = serde_json::to_string(reply).unwrap();
            let back: Reply = serde_json::from_str(&json).unwrap();
            assert_eq!(reply, &back);
        }
    }

    #[test]
    fn socket_path_uses_wayland_display() {
        // SAFETY: single-threaded test; no other thread reads the env here.
        unsafe { std::env::set_var("XDG_RUNTIME_DIR", "/run/user/1000") };
        assert_eq!(
            socket_path("wayland-1"),
            PathBuf::from("/run/user/1000/driftwm/ipc-wayland-1.sock")
        );
    }
}
