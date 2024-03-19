use crate::view::RootView;
use crate::widget::Constraints;
use crate::widget::{RootWidget, Widget};
use parley::FontContext;
use ralaire_core::{
    event::{self, EventCx},
    AppMessage, DebugLayer, InternalMessage, Point, RenderCommand, RenderCx, Renderer, RoundedRect,
    Shape, WidgetIdPath, WindowSize,
};
use ralaire_vello::RenderEngine;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;
pub struct AppWindow<'a, Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    size: WindowSize,
    event_cx: EventCx<AppMessage<Message>>,
    pub root_widget: RootWidget<Message>,
    pub root_view: RootView<Message>,
    cursor_pos: Point,
    focused_widget: WidgetIdPath,
    hovered_widget: WidgetIdPath,
    bounds_tree: Vec<(WidgetIdPath, RoundedRect)>,
    render_engine: RenderEngine<'a>,
    command_lists: Vec<Vec<RenderCommand>>,
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
            .with_min_inner_size(winit::dpi::PhysicalSize::new(200, 200))
            .with_title(title);
        let window = event_loop.create_window(window_attributes).unwrap();
        let size: WindowSize = window.inner_size().into();
        let root_widget: RootWidget<Message> = root_view.build_widget();
        let root_widget_id = root_widget.id;
        let bounds_tree = root_widget.bounds_tree(Vec::new(), Point::ZERO);

        debug.startup_finished();
        AppWindow {
            size,
            event_cx: EventCx::new(),
            root_widget,
            root_view,
            cursor_pos: Point::ZERO,
            focused_widget: vec![root_widget_id],
            hovered_widget: vec![root_widget_id],
            bounds_tree,
            render_engine: RenderEngine::new(window, size).await,
            command_lists: vec![],
        }
    }
    pub fn id(&self) -> winit::window::WindowId {
        self.render_engine.window().id()
    }

    pub fn set_title(&mut self, title: String) {
        self.render_engine.window().set_title(title.as_str())
    }

    pub fn set_root_view(&mut self, root_view: RootView<Message>) {
        self.root_view = root_view
    }

    pub fn cursor_pos(&self) -> Point {
        self.cursor_pos
    }

    fn set_cursor_pos(&mut self, cursor_pos: Point) {
        self.cursor_pos = cursor_pos
    }

    pub fn messages(&mut self) -> Vec<Message> {
        let mut internal_messages: Vec<AppMessage<Message>> = vec![];
        let messages = self
            .event_cx
            .messages()
            .into_iter()
            .filter_map(|message| match message {
                AppMessage::Internal(message) => {
                    internal_messages.push(AppMessage::Internal(message));
                    None
                }
                AppMessage::User(message) => Some(message),
            })
            .collect();
        let _: Vec<()> = internal_messages
            .into_iter()
            .map(|message| self.event_cx.add_message(message))
            .collect();
        messages
    }

    pub fn request_redraw(&self) {
        self.render_engine.window().request_redraw();
    }

    pub fn resize(
        &mut self,
        new_size: impl Into<WindowSize>,
        font_cx: &mut FontContext,
        debug: &mut DebugLayer,
    ) {
        let size: WindowSize = new_size.into();
        tracing::warn!("Resize requested, new_size: {size:?}");
        self.size = size;
        debug.layout_started();

        // TODO: do layout
        self.root_widget.layout(
            Constraints {
                min_size: size.into(),
                max_size: size.into(),
            },
            font_cx,
        );
        debug.layout_finished();
        self.render_engine.resize(self.size);
    }
    pub fn layout(&mut self, font_cx: &mut FontContext) {
        self.root_widget.layout(
                Constraints {
                    min_size: self.size.into(),
                    max_size: self.size.into(),
                },
                font_cx,
            );
    }
    pub fn render(&mut self, debug: &mut DebugLayer) {
        self.render_engine.render(self.command_lists.clone(), debug)
    }
    pub fn paint(&mut self) {
        let mut render_cx = RenderCx::new();
        self.root_widget.render(&mut render_cx);
        self.command_lists = render_cx.get_command_lists();
    }
    pub fn event(&mut self, event: event::window::Event) -> Vec<Message> {
        self.bounds_tree = self.root_widget.bounds_tree(Vec::new(), Point::ZERO);
        let mut changed_hover = false;
        if let event::window::Event::Mouse(mouse_event) = event.clone() {
            if let event::mouse::Event::Move { position, .. } = mouse_event {
                self.set_cursor_pos(Point::new(position.x, position.y));
                let previous = self.hovered_widget.clone();
                let _: Vec<()> = self
                    .bounds_tree
                    .iter()
                    .map(|(id_path, bounds)| {
                        if bounds.contains(position) {
                            self.hovered_widget = id_path.clone();
                        }
                    })
                    .collect();
                if previous != self.hovered_widget {
                    changed_hover = true;
                    self.root_widget.send_hover(false, previous);
                    self.render_engine.window().request_redraw();
                }
                // tracing::debug!("Hovered widget: {:?}", self.hovered_widget);
            }

            if let event::mouse::Event::Press {
                position,
                button: _,
            } = mouse_event
            {
                let _: Vec<()> = self
                    .bounds_tree
                    .iter()
                    .map(|(id_path, bounds)| {
                        if bounds.contains(position) {
                            self.focused_widget = id_path.clone();
                        }
                    })
                    .collect();
            }
        }
        if let Some(widget_event) = event::widget_event_from_window_event(event, Point::ZERO) {
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
        self.render_engine
            .window()
            .set_cursor(winit::window::Cursor::Icon(self.event_cx.cursor().into()));
        let messages = self.messages();

        let internal_messages = self.event_cx.messages();

        for message in internal_messages {
            match message {
                AppMessage::Internal(internal_mesage) => match internal_mesage {
                    InternalMessage::DragResizeWindow(direction) => {
                        let _ = self
                            .render_engine
                            .window()
                            .drag_resize_window(direction.into());
                    }
                    InternalMessage::DragMoveWindow => {
                        let _ = self.render_engine.window().drag_window();
                    }
                },
                AppMessage::User(_) => unreachable!(),
            }
        }
        messages
    }
}
