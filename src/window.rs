use std::sync::Arc;

use crate::app::InternalMessage;
use crate::event::WidgetEvent;
use crate::event::{self, EventContext};
use crate::renderer::RenderEngine;
use crate::view::{RootView, View};
use crate::widget::{RootWidget, Widget, WidgetIdPath};
use parley::FontContext;
use vello::peniko::kurbo::{Point, Rect, Size};
use winit::dpi::PhysicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window as WinitWindow, WindowId};
pub struct Window<State: 'static, V: View<State>> {
    // id: String,
    winit_window: Arc<WinitWindow>,
    physical_size: PhysicalSize<u32>,
    logical_size: Size,
    pub scale_factor: f64,
    event_context: EventContext,
    pub root_widget: RootWidget<State>,
    pub root_view: RootView<State, V>,
    cursor_pos: Point,
    hovered_widget: WidgetIdPath,
    bounds_tree: Vec<(WidgetIdPath, Rect)>,
    render_engine: RenderEngine,
}

impl<State: 'static, V> Window<State, V>
where
    V: View<State>,
    V::Element: Widget<State>,
{
    pub async fn new(
        event_loop: &ActiveEventLoop,
        root_view: RootView<State, V>,
        _id: String,
    ) -> Self {
        let window_attributes = WinitWindow::default_attributes()
            .with_decorations(false)
            .with_transparent(true)
            .with_min_inner_size(winit::dpi::LogicalSize::new(200, 200));
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        let physical_size = window.inner_size();
        let logical_size = (physical_size.width as f64, physical_size.height as f64).into();

        let mut root_widget = root_view.build_widget();
        let root_child_id = root_widget.child().id;
        Self {
            // id,
            winit_window: window.clone(),
            physical_size,
            logical_size,
            scale_factor: 1.0,
            event_context: EventContext::new(window.clone()),
            root_widget,
            root_view,
            cursor_pos: Point::ZERO,
            // FIXME: This shouldn't be necessary
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

    // pub fn id(&self) -> &str {
    //     &self.id
    // }

    pub fn winit_id(&self) -> WindowId {
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

    pub fn resize(&mut self, new_size: PhysicalSize<u32>, font_context: &mut FontContext) {
        self.physical_size = new_size;
        self.logical_size = Size::new(
            self.physical_size.width as f64 / self.scale_factor,
            self.physical_size.height as f64 / self.scale_factor,
        );
        self.layout(font_context);
        self.render_engine
            .resize(self.physical_size.width, self.physical_size.height);
        self.bounds_tree = self.root_widget.bounds_tree(Vec::new(), Point::ZERO);
    }

    pub fn layout(&mut self, font_context: &mut FontContext) {
        self.root_widget
            .layout(self.logical_size.into(), font_context);
        self.bounds_tree = self.root_widget.bounds_tree(Vec::new(), Point::ZERO);
    }

    pub fn paint(&mut self) {
        let mut scene = vello::Scene::new();
        self.root_widget.paint(&mut scene);
        // draw debug layout thing
        use rand::{Rng, SeedableRng};
        use vello::kurbo::Affine;
        use vello::kurbo::Stroke;
        use vello::peniko::Color;
        for (id, bounds) in &self.bounds_tree {
            let mut rng = rand::rngs::StdRng::seed_from_u64(id.last().unwrap().to_raw() << 3);
            scene.stroke(
                &Stroke::default(),
                Affine::default(),
                Color::rgb8(rng.gen(), rng.gen(), rng.gen()).multiply_alpha(0.8),
                None,
                &bounds.inset(-0.5),
            );
        }
        let mut scaled_scene = vello::Scene::new();
        scaled_scene.append(&scene, Some(Affine::scale(self.scale_factor)));
        self.render_engine.render(&scaled_scene);
    }

    pub fn reconciliate(&mut self, view: V) {
        let new = RootView::new(view);
        new.reconciliate(&self.root_view, &mut self.root_widget);
        self.root_view = new;
    }

    pub fn widget_event(
        &mut self,
        event: WidgetEvent,
        state: &mut State,
        should_close: &mut bool,
        state_changed: &mut bool,
    ) {
        // eprintln!("{:#?}", self.root_widget.child());

        if let WidgetEvent::Mouse(event::mouse::Event::Move { position, .. }) = event.clone() {
            self.set_cursor_pos(position);
        }
        let previous = self.hovered_widget.clone();
        let _: Vec<()> = self
            .bounds_tree
            .iter()
            .map(|(id_path, bounds)| {
                if bounds.contains(self.cursor_pos) {
                    self.hovered_widget.clone_from(id_path);
                }
            })
            .collect();
        if previous != self.hovered_widget {
            self.root_widget.send_hover(false, previous);
            self.root_widget
                .send_hover(true, self.hovered_widget.clone());
            self.winit_window.request_redraw();
        }
        self.root_widget.send_event(
            event,
            &mut self.event_context,
            self.hovered_widget.clone(),
            state,
        );
        self.winit_window.set_cursor(self.event_context.cursor());

        if self.event_context.repaint_needed {
            self.winit_window.request_redraw();
            self.event_context.repaint_needed = false;
        }
        for message in self.event_context.drain_internal_messages() {
            match message {
                InternalMessage::DragResizeWindow(_) => {}
                InternalMessage::DragMoveWindow => {}
                InternalMessage::MinimiseWindow => self.winit_window.set_minimized(true),
                InternalMessage::MaximiseWindow => self
                    .winit_window
                    .set_maximized(!self.winit_window.is_maximized()),
                InternalMessage::CloseWindow => *should_close = true,
                InternalMessage::TitleChanged(title) => self.winit_window.set_title(title.as_str()),
            }
        }
        *state_changed = self.event_context.state_changed;
        self.event_context.state_changed = false;
    }
}
