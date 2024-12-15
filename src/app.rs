use winit::window::ResizeDirection;

use crate::event::window::Event;
use crate::event::window_event;
use crate::view::View;
use crate::widget::Widget;
use crate::window::Window;
use crate::{event::widget_event_from_window_event, view::RootView};
use parley::FontContext;
use winit::{
    application::ApplicationHandler,
    error::EventLoopError,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

pub trait WidgetView<State: 'static>: View<State, Element = Self::Widget> {
    type Widget: Widget<State>;
}

impl<State: 'static, W, V> WidgetView<State> for V
where
    V: View<State, Element = W>,
    W: Widget<State>,
{
    type Widget = W;
}

#[derive(Debug, Clone)]
pub enum InternalMessage {
    TitleChanged(String),
    MinimiseWindow,
    MaximiseWindow,
    CloseWindow,
    DragResizeWindow(ResizeDirection),
    DragMoveWindow,
}

pub struct App<State, Logic> {
    state: State,
    logic: Logic,
}
impl<'a, State, Logic, V> App<State, Logic>
where
    State: 'static,
    Logic: FnMut(&mut State) -> V,
    V: View<State>,
    V::Element: Widget<State>,
{
    pub fn new(state: State, logic: Logic) -> Self {
        Self { state, logic }
    }

    pub fn run(self) -> Result<(), EventLoopError> {
        InternalApp::run(self.state, self.logic)
    }
}

// pub struct AppContext<State> {
//     pub state: State,
//     _event_loop_proxy: EventLoopProxy<()>,
// }
// impl<State> AppContext<State> {}

pub struct InternalApp<State, Logic, V>
where
    State: 'static,
    Logic: FnMut(&mut State) -> V,
    V: View<State>,
    // V::Element: Widget<State>,
{
    state: State,
    logic: Logic,
    runner: tokio::runtime::Runtime,
    font_context: FontContext,
    windows: Vec<Window<State, V>>,
}

impl<'a, State, Logic, V> InternalApp<State, Logic, V>
where
    State: 'static,
    Logic: FnMut(&mut State) -> V,
    V: View<State>,
    V::Element: Widget<State>,
{
    pub fn run(state: State, logic: Logic) -> Result<(), EventLoopError> {
        let runner = tokio::runtime::Runtime::new().unwrap();
        let event_loop = EventLoop::with_user_event().build().unwrap();
        let _event_loop_proxy = event_loop.create_proxy();
        let mut app = Self {
            state,
            logic,
            runner,
            font_context: FontContext::default(),
            windows: vec![],
        };
        event_loop.run_app(&mut app)
    }
}

impl<'a, State, Logic, V> ApplicationHandler<()> for InternalApp<State, Logic, V>
where
    State: 'static,
    Logic: FnMut(&mut State) -> V,

    V: View<State>,
    V::Element: Widget<State>,
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        tracing::trace!("Resumed the event loop");
        let view = (self.logic)(&mut self.state);
        let root_view = RootView::new(view);
        let w = self
            .runner
            .block_on(Window::new(event_loop, root_view, "TODO: id?".to_owned()));
        self.windows.push(w);
    }
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, message: ()) {
        tracing::trace!("User message: {message:?}");
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        // tracing::trace!("window event: {event:?}, with id {window_id:?}");
        let (idx, window) = match self
            .windows
            .iter_mut()
            .enumerate()
            .find(|(_, window)| window.winit_id() == window_id)
        {
            Some((idx, window)) => (idx, window),
            None => return,
        };
        let event = window_event(&event, window.cursor_pos(), window.scale_factor());
        let event = match event {
            Some(event) => event,
            None => return,
        };

        match event {
            Event::CloseRequested => {
                tracing::trace!("Closing Window={:?}", window.winit_id());
                if self.windows.len() == 1 {
                    self.windows.pop();
                    event_loop.exit()
                } else {
                    self.windows.remove(idx);
                }
            }
            Event::Resized(size) => {
                tracing::trace!("{:?}", size);
                window.resize(size, &mut self.font_context);
            }
            Event::ScaleFactorChanged(scale_factor) => window.scale_factor = scale_factor,
            Event::RedrawRequested => {
                window.paint();
            }
            Event::Keyboard(_) | Event::Mouse(_) | Event::Touch(_) => {
                let mut should_close = false;
                let mut state_changed = false;

                window.widget_event(
                    widget_event_from_window_event(event).unwrap(),
                    &mut self.state,
                    &mut should_close,
                    &mut state_changed,
                );
                if state_changed {
                    // TODO: Reconciliate window list properly
                    tracing::trace!("state changed");
                    let view = (self.logic)(&mut self.state);
                    window.reconciliate(view);
                    window.layout(&mut self.font_context);
                    window.request_redraw();
                }

                if should_close {
                    if self.windows.len() == 1 {
                        self.windows.pop();
                        event_loop.exit()
                    } else {
                        self.windows.remove(idx);
                    }
                }
            }
        }
    }
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {}

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {}
}
