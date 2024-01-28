#![feature(async_closure)]
use ralaire_core::{
    Affine, DebugLayer, Rect, RenderCommand, Renderer, RoundedRect, TextLayout, WindowSize,
};
// use ralaire_core::{Color, Stroke};
use rayon::prelude::*;
use std::sync::Arc;
use vello::glyph::{skrifa::raw::FontRef, GlyphContext};
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
        let mut render_cx = vello::util::RenderContext::new().unwrap();
        let surface = render_cx
            .create_surface(window.clone(), size.width, size.height)
            .await
            .unwrap();
        let renderer = vello::Renderer::new(
            &render_cx.devices[surface.dev_id].device,
            vello::RendererOptions {
                surface_format: Some(surface.format),
                use_cpu: false,
                antialiasing_support: vello::AaSupport::all(),
            },
        )
        .unwrap();

        RenderEngine {
            size,
            render_cx,
            surface,
            window: window,
            renderer,
        }
    }
    pub fn window(&self) -> &Window {
        &self.window
    }
    fn render_command_list(command_list: Vec<RenderCommand>) -> vello::SceneFragment {
        let mut fragment = vello::SceneFragment::new();
        let mut builder = vello::SceneBuilder::for_fragment(&mut fragment);
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
                    // // tracing::debug!("Clipped translated bounds: {:#?}", trans);
                    // builder.stroke(
                    //     &Stroke::default().with_dashes(0., [3., 3.]),
                    //     transform,
                    //     &Color::DARK_MAGENTA,
                    //     None,
                    //     &trans,
                    // );
                    builder.push_layer(blend, 1.0, transform, &trans);
                }
                RenderCommand::PopLayer => {
                    absolute_bounds.pop();
                    builder.pop_layer();
                }
                RenderCommand::FillShape {
                    transform,
                    shape,
                    brush,
                } => {
                    let transform =
                        transform.pre_translate(absolute_bounds.last().unwrap().origin().to_vec2());
                    builder.fill(
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
                    builder.stroke(&style, transform, &brush, None, &shape);
                }
                RenderCommand::DrawSvg {
                    transfomr: _,
                    svg_data: _,
                } => todo!(),
                RenderCommand::DrawText { layout } => {
                    let transform =
                        Affine::translate(absolute_bounds.last().unwrap().origin().to_vec2());
                    let mut gcx = GlyphContext::new();
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
                            let font_ref = font.as_ref();
                            if let Ok(font_ref) = FontRef::from_index(font_ref.data, font.index()) {
                                let style = glyph_run.style();
                                let vars: [(&str, f32); 0] = [];
                                let mut gp = gcx.new_provider(&font_ref, font_size, false, vars);
                                for glyph in glyph_run.glyphs() {
                                    if let Some(fragment) = gp.get(glyph.id, Some(&style.brush.0)) {
                                        let gx = x + glyph.x;
                                        let gy = y - glyph.y;
                                        let xform = Affine::translate((gx as f64, gy as f64))
                                            * Affine::scale_non_uniform(1.0, -1.0);
                                        builder.append(&fragment, Some(transform * xform));
                                    }
                                    x += glyph.advance;
                                }
                            }
                        }
                    }
                }
            }
        }
        fragment
    }
}
#[allow(unused_mut)]
impl<'a> Renderer for RenderEngine<'a> {
    async fn render(&mut self, mut command_lists: Vec<Vec<RenderCommand>>, debug: &mut DebugLayer) {
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
        let mut builder = vello::SceneBuilder::for_scene(&mut scene);
        debug.encode_started();
        // let mut join_handles: Vec<_> = vec![];
        // for commands in command_lists {
        //     join_handles.push(tokio::spawn(async move {
        //         RenderEngine::render_command_list(commands)
        //     }))
        // }
        // // tracing::info!("join handles: {}", join_handles.len());
        // let fragments = futures::future::try_join_all(join_handles).await.unwrap();

        let fragments = command_lists
            .into_par_iter()
            .map(RenderEngine::render_command_list)
            .collect::<Vec<_>>();

        // let fragments = command_lists
        //     .into_iter()
        //     .map(RenderEngine::render_command_list)
        //     .collect::<Vec<_>>();

        debug.encode_finished();
        for fragment in fragments {
            builder.append(&fragment, None);
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
