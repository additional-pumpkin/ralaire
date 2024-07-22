use std::sync::Arc;

use crate::event::window::Event;
use crate::renderer::RenderEngine;
use crate::view::RootView;
use crate::widget::{RootWidget, Widget};
use crate::{
    event::{self, EventCx},
    DebugLayer, InternalMessage, WidgetIdPath, WindowSize,
};
use parley::FontContext;
use vello::peniko::kurbo::{Point, Rect, Size};
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;
pub struct AppWindow<'a, Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    window: Arc<Window>,
    physical_size: WindowSize,
    logical_size: Size,
    scale_factor: f64,
    event_cx: EventCx<Message>,
    pub root_widget: RootWidget<Message>,
    pub root_view: RootView<Message>,
    cursor_pos: Point,
    focused_widget: WidgetIdPath,
    hovered_widget: WidgetIdPath,
    bounds_tree: Vec<(WidgetIdPath, Rect)>,
    render_engine: RenderEngine<'a>,
}

impl<'a, Message> AppWindow<'a, Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    pub async fn new(
        event_loop: &ActiveEventLoop,
        title: String,
        root_view: RootView<Message>,
        debug: &mut DebugLayer,
    ) -> AppWindow<'a, Message> {
        debug.startup_started();
        let window_attributes = Window::default_attributes()
            .with_decorations(false)
            .with_transparent(true)
            .with_min_inner_size(winit::dpi::LogicalSize::new(200, 200))
            .with_title(title);
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        let physical_size: WindowSize = window.inner_size().into();
        let logical_size = physical_size.into();
        let mut root_widget: RootWidget<Message> = root_view.build_widget();
        let root_child_id = root_widget.child().id;
        debug.startup_finished();
        AppWindow {
            window: window.clone(),
            physical_size,
            logical_size,
            scale_factor: 1.0,
            event_cx: EventCx::new(),
            root_widget,
            root_view,
            cursor_pos: Point::ZERO,
            // FIXME: This shouldn't be necessary
            focused_widget: vec![root_child_id],
            hovered_widget: vec![root_child_id],
            bounds_tree: vec![],
            render_engine: RenderEngine::new(window.clone(), physical_size).await,
        }
    }
    pub fn id(&self) -> winit::window::WindowId {
        self.window.id()
    }

    pub fn set_root_view(&mut self, root_view: RootView<Message>) {
        self.root_view = root_view
    }

    pub fn scale_factor(&self) -> f64 {
        self.scale_factor
    }

    pub fn cursor_pos(&self) -> Point {
        self.cursor_pos
    }

    fn set_cursor_pos(&mut self, cursor_pos: Point) {
        self.cursor_pos = cursor_pos
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }
    pub fn resize(
        &mut self,
        new_size: impl Into<WindowSize>,
        font_cx: &mut FontContext,
        debug: &mut DebugLayer,
    ) {
        self.physical_size = new_size.into();
        self.logical_size = Size::new(
            self.physical_size.width as f64 / self.scale_factor,
            self.physical_size.height as f64 / self.scale_factor,
        );
        debug.layout_started();
        // TODO: do layout
        self.root_widget.layout(self.logical_size, font_cx);
        debug.layout_finished();
        self.render_engine.resize(self.physical_size);
        self.bounds_tree = self.root_widget.bounds_tree(Vec::new(), Point::ZERO);
    }
    pub fn layout(&mut self, font_cx: &mut FontContext) {
        self.root_widget.layout(self.logical_size.into(), font_cx);
    }
    pub fn paint(&mut self, debug: &mut DebugLayer) {
        let mut scene = vello::Scene::new();
        debug.draw_started();
        self.root_widget.paint(&mut scene);
        debug.draw_finished();
        debug.render_started();
        // draw debug layout thing
        use rand::{Rng, SeedableRng};
        use vello::kurbo::{Affine, Stroke};
        use vello::peniko::Color;
        for (id, bounds) in &self.bounds_tree {
            let mut rng = rand::rngs::StdRng::seed_from_u64(id.last().unwrap().to_raw() << 3);
            scene.stroke(
                &Stroke::default().with_dashes(0., [3., 3.]),
                Affine::default(),
                Color::rgb8(rng.gen(), rng.gen(), rng.gen()).with_alpha_factor(0.8),
                None,
                &bounds.inset(-0.5),
            );
        }

        self.render_engine.render(&scene, self.scale_factor);
        debug.render_finished();
    }
    pub fn event(
        &mut self,
        event: Event,
        event_loop: &ActiveEventLoop,
        font_cx: &mut FontContext,
        debug: &mut DebugLayer,
    ) -> Vec<Message> {
        // eprintln!("{:#?}", self.root_widget.child());
        match event {
            Event::CloseRequested => {
                tracing::trace!("Closing Window={:?}", self.window.id());
                event_loop.exit()
            }
            Event::Resized(size) => {
                tracing::trace!("{:?}", size);
                self.resize(size, font_cx, debug);
            }
            Event::ScaleFactorChanged(scale_factor) => self.scale_factor = scale_factor,
            Event::RedrawRequested => {
                self.paint(debug);
                debug.log();
            }
            _ => {
                let mut changed_hover = false;
                if let Event::Mouse(mouse_event) = event.clone() {
                    if let event::mouse::Event::Move { position, .. } = mouse_event {
                        self.set_cursor_pos(position);
                        let previous = self.hovered_widget.clone();
                        let _: Vec<()> = self
                            .bounds_tree
                            .iter()
                            .map(|(id_path, bounds)| {
                                if bounds.contains(position) {
                                    self.hovered_widget.clone_from(id_path);
                                }
                            })
                            .collect();
                        if previous != self.hovered_widget {
                            changed_hover = true;
                            self.root_widget.send_hover(false, previous);
                            self.window.request_redraw();
                        }
                    }

                    if let event::mouse::Event::Press {
                        position,
                        button: _,
                    } = mouse_event
                    {
                        // FIXME: This is just not great
                        let _: Vec<()> = self
                            .bounds_tree
                            .iter()
                            .map(|(id_path, bounds)| {
                                if bounds.contains(position) {
                                    self.focused_widget.clone_from(id_path);
                                }
                            })
                            .collect();
                    }
                }
                if let Some(widget_event) =
                    event::widget_event_from_window_event(event, Point::ZERO)
                {
                    self.root_widget.send_event(
                        widget_event,
                        &mut self.event_cx,
                        self.focused_widget.clone(),
                    );
                    if changed_hover {
                        self.root_widget
                            .send_hover(true, self.hovered_widget.clone());
                    }
                }
                self.window.set_cursor(self.event_cx.cursor());
            }
        }
        if self.event_cx.repaint_needed {
            dbg!();
            self.window.request_redraw();
            self.event_cx.repaint_needed = false;
        }
        for message in self.event_cx.drain_internal_messages() {
            match message {
                InternalMessage::DragResizeWindow(direction) => {
                    let _ = self.window.drag_resize_window(direction);
                }
                InternalMessage::DragMoveWindow => {
                    let _ = self.window.drag_window();
                }
                InternalMessage::MinimiseWindow => self.window.set_minimized(true),
                InternalMessage::MaximiseWindow => {
                    self.window.set_maximized(!self.window.is_maximized())
                }
                InternalMessage::CloseWindow => event_loop.exit(),
                InternalMessage::TitleChanged(title) => self.window.set_title(title.as_str()),
            }
        }
        self.event_cx.drain_user_messages().collect()
    }
}
