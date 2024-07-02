use std::sync::Arc;

use crate::{DebugLayer, WidgetId, WindowSize};
use peniko::kurbo::{Affine, BezPath, Rect, Shape, Size, Stroke};
use peniko::{BlendMode, Brush, Color};
extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;
use parley::Layout;
use peniko::Image;
use rand::{Rng, SeedableRng};
use vello_svg::usvg;

#[derive(Debug, Clone)]
pub enum RenderCommand {
    PushWidget {
        id: WidgetId,
        bounds: Rect,
    },
    PopWidget,
    PushLayer {
        blend: BlendMode,
        transform: Affine,
        clip: BezPath,
    },
    PopLayer,
    FillShape {
        shape: BezPath,
        brush: Brush,
    },
    StrokeShape {
        style: Stroke,
        shape: BezPath,
        brush: Brush,
    },
    DrawImage {
        image: Image,
    },
    DrawSvg {
        svg: usvg::Tree,
    },
    DrawText {
        layout: TextLayout,
    },
}

#[derive(Clone)]
pub enum TextLayout {
    ParleyLayout(Layout<Brush>),
}

impl core::fmt::Debug for TextLayout {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ParleyLayout(arg0) => f
                .debug_tuple("ParleyLayout")
                .field(&Size::new(arg0.width() as f64, arg0.height() as f64))
                .finish(),
        }
    }
}

pub struct RenderCx {
    pub command_stack: Vec<RenderCommand>,
}

impl RenderCx {
    pub fn new() -> Self {
        Self {
            command_stack: vec![],
        }
    }

    pub fn push_widget(&mut self, id: WidgetId, bounds: Rect) {
        self.command_stack
            .push(RenderCommand::PushWidget { id, bounds });
    }
    pub fn pop_widget(&mut self) {
        self.command_stack.push(RenderCommand::PopWidget);
    }

    pub fn clear(&mut self) {
        self.command_stack.clear();
    }
}

#[derive(Debug, Clone)]
pub struct PaintCx {
    pub command_stack: Vec<RenderCommand>,
}
impl PaintCx {
    pub fn new() -> Self {
        Self {
            command_stack: vec![],
        }
    }

    pub fn push_layer(&mut self, blend: BlendMode, transform: Affine, bounds: BezPath) {
        self.command_stack.push(RenderCommand::PushLayer {
            blend,
            transform,
            clip: bounds,
        });
    }
    pub fn pop_layer(&mut self) {
        self.command_stack.push(RenderCommand::PopLayer);
    }
    pub fn fill_shape(&mut self, shape: &impl Shape, background: impl Into<Brush>) {
        self.command_stack.push(RenderCommand::FillShape {
            shape: shape.into_path(0.1),
            brush: background.into(),
        });
    }

    pub fn stroke_shape(&mut self, style: Stroke, shape: &impl Shape, color: impl Into<Brush>) {
        self.command_stack.push(RenderCommand::StrokeShape {
            style,
            shape: shape.into_path(0.1),
            brush: color.into(),
        });
    }

    pub fn draw_image(&mut self, image: Image) {
        self.command_stack.push(RenderCommand::DrawImage { image })
    }

    pub fn draw_svg(&mut self, svg: usvg::Tree) {
        self.command_stack.push(RenderCommand::DrawSvg { svg })
    }

    // TODO: Needs abstractions and easier API
    pub fn draw_text(&mut self, layout: TextLayout) {
        self.command_stack.push(RenderCommand::DrawText { layout });
    }
}
pub struct RenderEngine<'a> {
    size: WindowSize,
    render_cx: vello::util::RenderContext,
    surface: vello::util::RenderSurface<'a>,
    renderer: vello::Renderer,
}

impl<'a> RenderEngine<'a> {
    pub async fn new(window: Arc<winit::window::Window>, size: WindowSize) -> RenderEngine<'a> {
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
            renderer,
        }
    }

    fn encode_commands(command_list: Vec<RenderCommand>, scale: Affine) -> vello::Scene {
        let mut fragment = vello::Scene::new();
        let mut transforms = vec![scale];
        let mut debug_bounds = vec![];
        for command in command_list {
            match command {
                RenderCommand::PushWidget { id, bounds } => {
                    let prev = *transforms.last().unwrap();
                    debug_bounds.push((id, bounds, prev));
                    transforms.push(prev * Affine::translate(bounds.origin().to_vec2()));
                }
                RenderCommand::PopWidget => {
                    transforms.pop();
                }
                RenderCommand::PushLayer {
                    blend,
                    transform,
                    mut clip,
                } => {
                    let prev = *transforms.last().unwrap();
                    clip.apply_affine(prev);
                    fragment.stroke(
                        &Stroke::new(5.),
                        Affine::default(),
                        Color::RED.with_alpha_factor(0.5),
                        None,
                        &clip,
                    );
                    fragment.push_layer(blend, 1.0, Affine::default(), &clip);
                    transforms.push(prev * transform);
                }
                RenderCommand::PopLayer => {
                    fragment.pop_layer();
                    transforms.pop();
                }
                RenderCommand::FillShape { shape, brush } => {
                    let transform = *transforms.last().unwrap();
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
                    shape,
                    brush,
                } => {
                    let transform = *transforms.last().unwrap();
                    fragment.stroke(&style, transform, &brush, None, &shape);
                }
                RenderCommand::DrawImage { image } => {
                    let transform = *transforms.last().unwrap();
                    fragment.draw_image(&image, transform);
                }
                RenderCommand::DrawText { layout } => {
                    let transform = *transforms.last().unwrap();
                    let TextLayout::ParleyLayout(layout) = layout;
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
                RenderCommand::DrawSvg { svg } => {
                    let transform = *transforms.last().unwrap();
                    let mut svg_fragment = vello::Scene::new();
                    vello_svg::render_tree(&mut svg_fragment, &svg);
                    fragment.append(&svg_fragment, Some(transform));
                }
            }
        }
        for (id, bounds, prev) in debug_bounds {
            let mut rng = rand::rngs::StdRng::seed_from_u64(id.to_raw() << 3);
            fragment.stroke(
                &Stroke::default().with_dashes(0., [3., 3.]),
                prev,
                Color::rgb8(rng.gen(), rng.gen(), rng.gen()).with_alpha_factor(0.8),
                None,
                &bounds.inset(-0.5),
            );
        }
        fragment
    }

    pub fn render(&mut self, command_list: Vec<RenderCommand>, scale: f64, debug: &mut DebugLayer) {
        let base_color = vello::peniko::Color::TRANSPARENT;
        let render_params = vello::RenderParams {
            base_color,
            width: self.size.width,
            height: self.size.height,
            antialiasing_method: vello::AaConfig::Area,
        };
        debug.encode_started();
        let scene = RenderEngine::encode_commands(command_list, Affine::scale(scale));
        debug.encode_finished();

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

    pub fn resize(&mut self, new_size: WindowSize) {
        self.size = new_size;
        self.render_cx
            .resize_surface(&mut self.surface, new_size.width, new_size.height)
    }
}
