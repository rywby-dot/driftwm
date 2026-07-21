use smithay::output::{Mode, Output, PhysicalProperties, Subpixel};
use smithay::reexports::wayland_server::backend::GlobalId;
use smithay::utils::{Size, Transform};

use crate::state::DriftWm;

/// Create a fake `HEADLESS-{n}` output the way the real backends do — mode,
/// wl_output global — then hand it to [`DriftWm::output_connected`] for the
/// backend-independent connect policy (layout position, per-output viewport
/// state, focus/pointer bootstrap, Space mapping). Skips only the renderer,
/// dmabuf global, and render timer a real backend also installs. Outputs tile
/// left-to-right by creation order. Returns the output plus its `GlobalId`, so
/// the fixture can later disable/remove the global on disconnect.
pub fn add_output(state: &mut DriftWm, n: u8, size: (u16, u16)) -> (Output, GlobalId) {
    let output = Output::new(
        format!("HEADLESS-{n}"),
        PhysicalProperties {
            size: (0, 0).into(),
            subpixel: Subpixel::Unknown,
            make: "driftwm".to_string(),
            model: "headless".to_string(),
            serial_number: n.to_string(),
        },
    );

    let mode = Mode {
        size: Size::from((i32::from(size.0), i32::from(size.1))),
        refresh: 60_000,
    };
    output.change_current_state(Some(mode), Some(Transform::Normal), None, None);
    output.set_preferred(mode);
    let global = output.create_global::<DriftWm>(&state.display_handle);

    state.output_connected(&output, &std::collections::HashMap::new());

    (output, global)
}
