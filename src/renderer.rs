use std::sync::Arc;

pub struct RenderEngine {
    width: u32,
    height: u32,
    render_context: vello::util::RenderContext,
    surface: vello::util::RenderSurface<'static>,
    renderer: vello::Renderer,
}

impl RenderEngine {
    pub async fn new(window: Arc<winit::window::Window>, width: u32, height: u32) -> Self {
        let window = Arc::new(window);
        let mut render_context = vello::util::RenderContext::new();
        let surface = render_context
            .create_surface(window.clone(), width, height, wgpu::PresentMode::Fifo)
            .await
            .unwrap();
        let renderer = vello::Renderer::new(
            &render_context.devices[surface.dev_id].device,
            vello::RendererOptions {
                surface_format: Some(surface.format),
                use_cpu: false,
                antialiasing_support: vello::AaSupport::all(),
                num_init_threads: None,
            },
        )
        .unwrap();

        RenderEngine {
            width,
            height,
            render_context,
            surface,
            renderer,
        }
    }

    pub fn render(&mut self, scene: &vello::Scene) {
        let base_color = vello::peniko::Color::TRANSPARENT;
        let render_params = vello::RenderParams {
            base_color,
            width: self.width,
            height: self.height,
            antialiasing_method: vello::AaConfig::Area,
            // debug: vello::DebugLayers::none(),
        };

        let surface_texture = self
            .surface
            .surface
            .get_current_texture()
            .expect("failed to get surface texture");
        let device_handle = &self.render_context.devices[self.surface.dev_id];
        {
            self.renderer
                .render_to_surface(
                    &device_handle.device,
                    &device_handle.queue,
                    &scene,
                    &surface_texture,
                    &render_params,
                )
                .expect("failed to render to surface");
        }
        surface_texture.present();
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.render_context
            .resize_surface(&mut self.surface, width, height)
    }
}
