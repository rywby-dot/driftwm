use smithay::backend::allocator::Fourcc;
use smithay::backend::renderer::element::texture::{TextureBuffer, TextureRenderElement};
use smithay::backend::renderer::element::{Element, Kind, RenderElement};
use smithay::backend::renderer::gles::{GlesError, GlesRenderer, GlesTexture};
use smithay::backend::renderer::{Bind as _, Color32F, Frame as _, Renderer as _};
use smithay::utils::user_data::UserDataMap;
use smithay::utils::{Logical, Physical, Point, Rectangle, Scale, Size, Transform};

use super::{OutputRenderElements, WindowTransformElement};

const CLOSE_SCALE: f64 = 0.8;
const DONE_EPSILON: f64 = 0.001;

/// Short-lived GPU snapshot used after the real window has already left the
/// stage. One texture and one affine render element keep closing animations
/// independent of client teardown and cheap to draw.
#[derive(Debug)]
pub(crate) struct ClosingSnapshot {
    buffer: TextureBuffer<GlesTexture>,
    output: String,
    geometry: Rectangle<i32, Physical>,
    logical_size: Size<i32, Logical>,
    camera: Point<f64, Logical>,
    zoom: f64,
    pinned: bool,
    progress: f64,
}

impl ClosingSnapshot {
    pub fn tick(&mut self, frame_factor: f64) {
        self.progress += (1.0 - self.progress) * frame_factor;
    }

    pub fn is_done(&self) -> bool {
        1.0 - self.progress <= DONE_EPSILON
    }

    fn render_element(
        &self,
        camera: Point<f64, Logical>,
        zoom: f64,
        output_scale: f64,
    ) -> OutputRenderElements {
        let alpha = (1.0 - self.progress).clamp(0.0, 1.0) as f32;
        let texture = TextureRenderElement::from_texture_buffer(
            self.geometry.loc.to_f64(),
            &self.buffer,
            Some(alpha),
            None,
            Some(self.logical_size),
            Kind::Unspecified,
        );
        let close_scale = 1.0 - (1.0 - CLOSE_SCALE) * self.progress;
        let zoom_ratio = if self.pinned { 1.0 } else { zoom / self.zoom };
        let camera_offset: Point<f64, Physical> = if self.pinned {
            Point::default()
        } else {
            Point::from((
                (self.camera.x - camera.x) * zoom * output_scale,
                (self.camera.y - camera.y) * zoom * output_scale,
            ))
        };
        let captured_center =
            self.geometry.loc.to_f64() + self.geometry.size.to_f64().to_point().downscale(2.0);
        let offset = camera_offset
            + captured_center
                .upscale(zoom_ratio)
                .upscale(1.0 - close_scale);
        OutputRenderElements::ClosingWindow(WindowTransformElement::new(
            texture,
            Point::default(),
            offset,
            Scale::from(zoom_ratio * close_scale),
        ))
    }
}

pub(crate) fn capture(
    renderer: &mut GlesRenderer,
    output: &str,
    output_scale: Scale<f64>,
    camera: Point<f64, Logical>,
    zoom: f64,
    pinned: bool,
    elements: &[OutputRenderElements],
) -> Result<Option<ClosingSnapshot>, GlesError> {
    let Some(geometry) = elements
        .iter()
        .map(|element| element.geometry(output_scale))
        .reduce(|a, b| a.merge(b))
        .filter(|geometry| geometry.size.w > 0 && geometry.size.h > 0)
    else {
        return Ok(None);
    };

    let buffer_size = geometry.size.to_logical(1).to_buffer(1, Transform::Normal);
    let mut texture =
        <GlesRenderer as smithay::backend::renderer::Offscreen<GlesTexture>>::create_buffer(
            renderer,
            Fourcc::Abgr8888,
            buffer_size,
        )?;
    {
        let mut target = renderer.bind(&mut texture)?;
        let mut frame = renderer.render(&mut target, geometry.size, Transform::Normal)?;
        frame.clear(
            Color32F::TRANSPARENT,
            &[Rectangle::from_size(geometry.size)],
        )?;

        // OutputRenderElements are front-to-back. An offscreen framebuffer is
        // painter's algorithm, so draw them in reverse.
        for element in elements.iter().rev() {
            let src = element.src();
            let mut dst = element.geometry(output_scale);
            dst.loc -= geometry.loc;
            let Some(mut damage) = Rectangle::from_size(geometry.size).intersection(dst) else {
                continue;
            };
            damage.loc -= dst.loc;
            let cache = UserDataMap::new();
            if element.is_framebuffer_effect() {
                element.capture_framebuffer(&mut frame, src, dst, &cache)?;
            }
            element.draw(&mut frame, src, dst, &[damage], &[], Some(&cache))?;
        }
        let _sync = frame.finish()?;
    }

    let buffer = TextureBuffer::from_texture(renderer, texture, 1, Transform::Normal, None);
    let logical_size = geometry
        .size
        .to_f64()
        .to_logical(output_scale)
        .to_i32_round();
    Ok(Some(ClosingSnapshot {
        buffer,
        output: output.to_owned(),
        geometry,
        logical_size,
        camera,
        zoom,
        pinned,
        progress: 0.0,
    }))
}

pub(crate) fn render_for_output(
    snapshots: &[ClosingSnapshot],
    output: &str,
    camera: Point<f64, Logical>,
    zoom: f64,
    output_scale: f64,
) -> Vec<OutputRenderElements> {
    snapshots
        .iter()
        .filter(|snapshot| snapshot.output == output)
        .map(|snapshot| snapshot.render_element(camera, zoom, output_scale))
        .collect()
}
