use std::sync::Arc;

use crate::WindowSize;
use vello::peniko::kurbo::Affine;
extern crate alloc;

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

    pub fn render(&mut self, fragment: &vello::Scene, scale: f64) {
        let base_color = vello::peniko::Color::TRANSPARENT;
        let render_params = vello::RenderParams {
            base_color,
            width: self.size.width,
            height: self.size.height,
            antialiasing_method: vello::AaConfig::Area,
        };
        let mut scene = vello::Scene::new();
        scene.append(fragment, Some(Affine::scale(scale)));

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
