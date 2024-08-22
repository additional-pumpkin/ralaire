use std::sync::Arc;

use crate::event::window::Event;
use crate::renderer::RenderEngine;
use crate::view::{RootView, View};
use crate::widget::{RootWidget, Widget, WidgetIdPath};
use crate::{
    event::{self, EventCx},
    DebugLayer, InternalMessage,
};
use parley::FontContext;
use vello::peniko::kurbo::{Point, Rect, Size};
use winit::dpi::PhysicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window as WinitWindow;
pub struct Window<'a, State> {
    winit_window: Arc<WinitWindow>,
    physical_size: PhysicalSize<u32>,
    logical_size: Size,
    scale_factor: f64,
    event_cx: EventCx,
    pub root_widget: RootWidget<State>,
    pub root_view: RootView<State>,
    cursor_pos: Point,
    focused_widget: WidgetIdPath,
    hovered_widget: WidgetIdPath,
    bounds_tree: Vec<(WidgetIdPath, Rect)>,
    render_engine: RenderEngine<'a>,
}

impl<'a, State> Window<'a, State>
where
    State: 'static,
{
    pub async fn new(
        event_loop: &ActiveEventLoop,
        title: String,
        root_view: RootView<State>,
        debug: &mut DebugLayer,
    ) -> Self {
        debug.startup_started();
        let window_attributes = WinitWindow::default_attributes()
            .with_decorations(false)
            .with_transparent(true)
            .with_min_inner_size(winit::dpi::LogicalSize::new(200, 200))
            .with_title(title);
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        let physical_size = window.inner_size();
        let logical_size = (physical_size.width as f64, physical_size.height as f64).into();
        let mut root_widget = root_view.build_widget();
        let root_child_id = root_widget.child().id;
        debug.startup_finished();
        Self {
            winit_window: window.clone(),
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
            render_engine: RenderEngine::new(
                window.clone(),
                physical_size.width,
                physical_size.height,
            )
            .await,
        }
    }

    pub fn id(&self) -> winit::window::WindowId {
        self.winit_window.id()
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
        self.winit_window.request_redraw();
    }

    pub fn resize(
        &mut self,
        new_size: PhysicalSize<u32>,
        font_cx: &mut FontContext,
        debug: &mut DebugLayer,
    ) {
        self.physical_size = new_size;
        self.logical_size = Size::new(
            self.physical_size.width as f64 / self.scale_factor,
            self.physical_size.height as f64 / self.scale_factor,
        );
        debug.layout_started();
        self.layout(font_cx);
        debug.layout_finished();
        self.render_engine
            .resize(self.physical_size.width, self.physical_size.height);
        self.bounds_tree = self.root_widget.bounds_tree(Vec::new(), Point::ZERO);
    }

    pub fn layout(&mut self, font_cx: &mut FontContext) {
        self.root_widget.layout(self.logical_size.into(), font_cx);
        self.bounds_tree = self.root_widget.bounds_tree(Vec::new(), Point::ZERO);
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
                &Stroke::default(),
                // .with_dashes(0., [3., 3.]),
                Affine::default(),
                Color::rgb8(rng.gen(), rng.gen(), rng.gen()).with_alpha_factor(0.8),
                None,
                &bounds.inset(-0.5),
            );
        }
        self.render_engine.render(&scene, self.scale_factor);
        debug.render_finished();
    }

    pub fn event<Logic, V>(
        &mut self,
        event: Event,
        event_loop: &ActiveEventLoop,
        font_cx: &mut FontContext,
        state: &mut State,
        logic: &mut Logic,
        debug: &mut DebugLayer,
    ) where
        Logic: FnMut(&mut State) -> V,
        V: View<State>,
    {
        // eprintln!("{:#?}", self.root_widget.child());
        match event {
            Event::CloseRequested => {
                tracing::trace!("Closing Window={:?}", self.winit_window.id());
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
                            self.winit_window.request_redraw();
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
                        state,
                    );
                    if changed_hover {
                        self.root_widget
                            .send_hover(true, self.hovered_widget.clone());
                    }
                }
                self.winit_window.set_cursor(self.event_cx.cursor());
            }
        }
        if self.event_cx.repaint_needed {
            dbg!();
            self.winit_window.request_redraw();
            self.event_cx.repaint_needed = false;
        }
        for message in self.event_cx.drain_internal_messages() {
            match message {
                InternalMessage::DragResizeWindow(direction) => {
                    let _ = self.winit_window.drag_resize_window(direction);
                }
                InternalMessage::DragMoveWindow => {
                    let _ = self.winit_window.drag_window();
                }
                InternalMessage::MinimiseWindow => self.winit_window.set_minimized(true),
                InternalMessage::MaximiseWindow => self
                    .winit_window
                    .set_maximized(!self.winit_window.is_maximized()),
                InternalMessage::CloseWindow => event_loop.exit(),
                InternalMessage::TitleChanged(title) => self.winit_window.set_title(title.as_str()),
            }
        }
        if self.event_cx.state_changed {
            tracing::trace!("state changed");
            let new = RootView::new(Box::new((logic)(state)));
            new.reconciliate(&self.root_view, &mut self.root_widget);
            self.root_view = new;
            self.layout(font_cx);
            self.request_redraw();
            self.event_cx.state_changed = false;
        }
    }
}
