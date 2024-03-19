use ralaire_core::{
    Affine, DebugLayer, Rect, RenderCommand, Renderer, RoundedRect, TextLayout, WindowSize,
};
use ralaire_core::{Color, Stroke};
use rayon::prelude::*;
use std::sync::Arc;
use winit::window::Window;
pub struct RenderEngine<'a> {
    size: WindowSize,
    render_cx: vello::util::RenderContext,
    surface: vello::util::RenderSurface<'a>,
    window: Arc<winit::window::Window>,
    renderer: vello::Renderer,
}

impl<'a> RenderEngine<'a> {
    pub async fn new(window: winit::window::Window, size: WindowSize) -> RenderEngine<'a> {
        let window = Arc::new(window);
        let mut render_cx = vello::util::RenderContext::new();
        let surface = render_cx
            .create_surface(
                window.clone(),
                size.width,
                size.height,
                wgpu::PresentMode::Mailbox,
            )
            .await
            .unwrap();
        let renderer = vello::Renderer::new(
            &render_cx.devices[surface.dev_id].device,
            vello::RendererOptions {
                surface_format: Some(surface.format),
                use_cpu: false,
                antialiasing_support: vello::AaSupport::all(),
                num_init_threads: None,
            },
        )
        .unwrap();

        RenderEngine {
            size,
            render_cx,
            surface,
            window,
            renderer,
        }
    }
    pub fn window(&self) -> &Window {
        &self.window
    }
    fn render_command_list(command_list: Vec<RenderCommand>) -> vello::Scene {
        let mut fragment = vello::Scene::new();
        let mut absolute_bounds: Vec<RoundedRect> = vec![];
        for command in command_list {
            match command {
                RenderCommand::PushLayer {
                    blend,
                    transform,
                    bounds,
                } => {
                    let trans = bounds
                        + absolute_bounds
                            .last()
                            .unwrap_or(&Rect::ZERO.to_rounded_rect(0.))
                            .origin()
                            .to_vec2();

                    absolute_bounds.push(trans);
                    // tracing::debug!("Clipped translated bounds: {:#?}", trans);
                    fragment.stroke(
                        &Stroke::default().with_dashes(0., [3., 3.]),
                        transform,
                        &Color::DARK_MAGENTA,
                        None,
                        &trans,
                    );
                    fragment.push_layer(blend, 1.0, transform, &trans);
                }
                RenderCommand::PopLayer => {
                    absolute_bounds.pop();
                    fragment.pop_layer();
                }
                RenderCommand::FillShape {
                    transform,
                    shape,
                    brush,
                } => {
                    let transform =
                        transform.pre_translate(absolute_bounds.last().unwrap().origin().to_vec2());
                    fragment.fill(
                        vello::peniko::Fill::NonZero,
                        transform,
                        &brush,
                        None,
                        &shape,
                    );
                }
                RenderCommand::StrokeShape {
                    style,
                    transform,
                    shape,
                    brush,
                } => {
                    let transform =
                        transform.pre_translate(absolute_bounds.last().unwrap().origin().to_vec2());
                    fragment.stroke(&style, transform, &brush, None, &shape);
                }
                RenderCommand::DrawSvg {
                    transfomr: _,
                    svg_data: _,
                } => todo!(),
                RenderCommand::DrawText { layout } => {
                    let transform =
                        Affine::translate(absolute_bounds.last().unwrap().origin().to_vec2());
                    let layout = match layout {
                        TextLayout::ParleyLayout(layout) => layout,
                    };
                    for line in layout.lines() {
                        for glyph_run in line.glyph_runs() {
                            let mut x = glyph_run.offset();
                            let y = glyph_run.baseline();
                            let run = glyph_run.run();
                            let font = run.font();
                            let font_size = run.font_size();
                            let style = glyph_run.style();
                            let synthesis = run.synthesis();
                            let glyph_xform = synthesis
                                .skew()
                                .map(|angle| Affine::skew(angle.to_radians().tan() as f64, 0.0));
                            let coords = run
                                .normalized_coords()
                                .iter()
                                .map(|coord| {
                                    vello::skrifa::instance::NormalizedCoord::from_bits(*coord)
                                })
                                .collect::<Vec<_>>();
                            fragment
                                .draw_glyphs(font)
                                .brush(&style.brush)
                                .transform(transform)
                                .glyph_transform(glyph_xform)
                                .font_size(font_size)
                                .normalized_coords(&coords)
                                .draw(
                                    vello::peniko::Fill::NonZero,
                                    glyph_run.glyphs().map(|glyph| {
                                        let gx = x + glyph.x;
                                        let gy = y - glyph.y;
                                        x += glyph.advance;
                                        vello::glyph::Glyph {
                                            id: glyph.id as _,
                                            x: gx,
                                            y: gy,
                                        }
                                    }),
                                );
                        }
                    }
                }
            }
        }
        fragment
    }
}
impl<'a> Renderer for RenderEngine<'a> {
    fn render(&mut self, command_lists: Vec<Vec<RenderCommand>>, debug: &mut DebugLayer) {
        tracing::debug!("Render requested with {} layers", command_lists.len());
        // tracing::debug!("Render requested with commands: {:#?}", command_lists);

        let base_color = vello::peniko::Color::TRANSPARENT;
        let render_params = vello::RenderParams {
            base_color,
            width: self.size.width,
            height: self.size.height,
            antialiasing_method: vello::AaConfig::Area,
        };
        let mut scene = vello::Scene::new();
        debug.encode_started();
        let fragments = command_lists
            .into_par_iter()
            .map(RenderEngine::render_command_list)
            .collect::<Vec<_>>();
        debug.encode_finished();
        for fragment in fragments {
            scene.append(&fragment, None);
        }

        let surface_texture = self
            .surface
            .surface
            .get_current_texture()
            .expect("failed to get surface texture");
        let device_handle = &self.render_cx.devices[self.surface.dev_id];
        {
            vello::block_on_wgpu(
                &device_handle.device,
                self.renderer.render_to_surface_async(
                    &device_handle.device,
                    &device_handle.queue,
                    &scene,
                    &surface_texture,
                    &render_params,
                ),
            )
            .expect("failed to render to surface");
        }
        surface_texture.present();
    }

    fn resize(&mut self, new_size: WindowSize) {
        self.size = new_size;
        self.render_cx
            .resize_surface(&mut self.surface, new_size.width, new_size.height)
    }
}
