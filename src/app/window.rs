use crate::app::state::AppMessage;
use crate::widget::{Constraints, RootWidget, Widget};
use parley::FontContext;
use ralaire_core::{
    event::{self, EventCx},
    DebugLayer, Id, IdPath, Point, RenderCommand, RenderCx, Renderer, RoundedRect, Shape,
    WindowSize,
};
use ralaire_vello::RenderEngine;
use winit::event_loop::EventLoop;
use winit::window::Window;
pub struct AppWindow<'a, Message>
where
    Message: std::fmt::Debug + Clone + 'static,
{
    render_engine: RenderEngine<'a>,
    command_lists: Vec<Vec<RenderCommand>>,
    cursor_pos: Point,
    focused_widget: IdPath,
    hovered_widget: IdPath,
    bounds_tree: Vec<(Vec<Id>, RoundedRect)>,
    size: WindowSize,
    root_widget: RootWidget<Message>,
    event_cx: EventCx<AppMessage<Message>>,
}

impl<'a, Message> AppWindow<'a, Message>
where
    Message: std::fmt::Debug + Clone + 'static,
{
    pub async fn new(
        event_loop: &EventLoop<Message>,
        title: String,
        debug: &mut DebugLayer,
    ) -> AppWindow<'a, Message> {
        debug.startup_started();
        let window = Window::builder()
            .with_decorations(false)
            .with_transparent(true)
            .with_title(title)
            .build(event_loop)
            .unwrap();
        let size: WindowSize = window.inner_size().into();
        let root_widget: RootWidget<Message> = RootWidget::new();

        debug.startup_finished();
        AppWindow {
            cursor_pos: Point::ZERO,
            render_engine: RenderEngine::new(window, size).await,
            command_lists: vec![],
            focused_widget: vec![root_widget.id],
            hovered_widget: vec![root_widget.id],
            bounds_tree: root_widget.bounds_tree(Vec::new(), Point::ZERO),
            size,
            root_widget,
            event_cx: EventCx::new(),
        }
    }
    pub fn id(&self) -> winit::window::WindowId {
        self.render_engine.window().id()
    }
    pub fn set_view(&mut self, view: impl Widget<Message> + 'static) {
        self.root_widget.set_view(view);
    }
    pub fn set_header(&mut self, header: impl Widget<Message> + 'static) {
        self.root_widget.set_header_middle(header);
    }
    pub fn set_title(&mut self, title: String) {
        self.render_engine.window().set_title(title.as_str());
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

    pub fn update(&mut self) {
        self.bounds_tree = self.root_widget.bounds_tree(Vec::new(), Point::ZERO);
        let _: Vec<()> = self
            .bounds_tree
            .iter()
            .map(|(id_path, bounds)| {
                if bounds.contains(self.cursor_pos) {
                    self.hovered_widget = id_path.clone();
                }
            })
            .collect();
        let hovered_stale = self
            .root_widget
            .send_hover(true, self.hovered_widget.clone());
        self.hovered_widget
            .truncate(self.hovered_widget.len() - hovered_stale);
    }

    pub fn request_redraw(&self) {
        self.render_engine.window().request_redraw();
    }

    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.render_engine.window().inner_size()
    }

    pub fn resize(
        &mut self,
        new_size: impl Into<WindowSize>,
        font_cx: &mut FontContext,
        debug: &mut DebugLayer,
    ) {
        // tracing::warn!("Resize requested, new_size: {new_size:?}");
        let size: WindowSize = new_size.into();
        self.size = size;
        debug.layout_started();
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
    pub async fn render(&mut self, debug: &mut DebugLayer) {
        self.render_engine
            .render(self.command_lists.clone(), debug)
            .await;
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
                    let _ = self.root_widget.send_hover(false, previous);
                    self.render_engine.window().request_redraw();
                }
                // tracing::debug!("Hovered widget: {:?}", self.hovered_widge);
            }

            if let event::mouse::Event::Release {
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
            let focused_stale = self.root_widget.send_event(
                widget_event,
                &mut self.event_cx,
                self.focused_widget.clone(),
            );
            self.focused_widget
                .truncate(self.focused_widget.len() - focused_stale);
            if changed_hover {
                let hovered_stale = self
                    .root_widget
                    .send_hover(true, self.hovered_widget.clone());
                self.hovered_widget
                    .truncate(self.hovered_widget.len() - hovered_stale);
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
                    super::state::InternalMessage::DragResizeWindow(direction) => {
                        let _ = self
                            .render_engine
                            .window()
                            .drag_resize_window(direction.into());
                    }
                    super::state::InternalMessage::DragMoveWindow => {
                        let _ = self.render_engine.window().drag_window();
                    }
                },
                AppMessage::User(_) => unreachable!(),
            }
        }
        messages
    }
}
