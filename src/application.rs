#[derive(Debug, Clone)]
pub enum InternalMessage {
    TitleChanged(String),
    MinimiseWindow,
    MaximiseWindow,
    CloseWindow,
    DragResizeWindow(ResizeDirection),
    DragMoveWindow,
}

use winit::window::ResizeDirection;

use crate::task::InternalTask;
use crate::view::View;
use crate::AnimationDirection;
use crate::{event::window_event, AnimationId, DebugLayer};
use crate::{view::RootView, window::Window};
use parley::FontContext;
use std::sync::{Arc, Mutex};
use winit::{
    application::ApplicationHandler,
    error::EventLoopError,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    window::WindowId,
};

pub struct Application<'a, State, Logic> {
    state: State,
    logic: Logic,
    pub event_loop_proxy: Option<EventLoopProxy<()>>,
    pub runner: tokio::runtime::Runtime,
    pub debug: DebugLayer,
    font_cx: FontContext,
    // animations_running: Arc<Mutex<Vec<(AnimationId, A::Message)>>>,
    main_window: Option<Window<'a, State>>,
}

impl<'a, State, Logic, V> Application<'a, State, Logic>
where
    State: 'static,
    Logic: FnMut(&mut State) -> V,
    V: View<State>,
{
    pub fn new(state: State, logic: Logic) -> Self {
        let runner = tokio::runtime::Runtime::new().unwrap();
        Application {
            state,
            logic,
            event_loop_proxy: None,
            runner,
            debug: DebugLayer::new(),
            font_cx: FontContext::default(),
            main_window: None,
        }
    }

    pub fn run(mut self) -> Result<(), EventLoopError> {
        let event_loop = EventLoop::with_user_event().build().unwrap();
        let event_loop_proxy = event_loop.create_proxy();
        self.event_loop_proxy = Some(event_loop_proxy);
        event_loop.run_app(&mut self)
    }
}

impl<'a, State, Logic, V> ApplicationHandler for Application<'a, State, Logic>
where
    State: 'static,
    Logic: FnMut(&mut State) -> V,
    V: View<State>,
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        tracing::trace!("Resumed the event loop");
        let first_view = (self.logic)(&mut self.state);
        let main_window = self.runner.block_on(Window::new(
            event_loop,
            "TODO: Make titles work?".to_owned(),
            RootView::new(Box::new(first_view)),
            &mut self.debug,
        ));
        self.main_window = Some(main_window);
    }
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, message: ()) {
        tracing::trace!("User message: {message:?}");
        // self.messages.push(message);
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let window = match &mut self.main_window {
            Some(window) => window,
            None => return,
        };
        if window.id() != window_id {
            return;
        }
        let event = window_event(&event, window.cursor_pos(), window.scale_factor());
        let event = match event {
            Some(event) => event,
            None => return,
        };
        self.debug.event_started();
        window.event(
            event,
            event_loop,
            &mut self.font_cx,
            &mut self.state,
            &mut self.logic,
            &mut self.debug,
        );
        let _proxy = self.event_loop_proxy.as_ref().unwrap().clone();
        self.debug.event_finished();
    }
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        // for command in commands {
        //     for task in command.tasks {
        //         match task {
        //             InternalTask::Animate {
        //                 mut animation,
        //                 tick_callback,
        //                 done_callback,
        //             } => {
        //                 let animations_running = self.animations_running.clone();
        //                 let proxy = self.event_loop_proxy.as_ref().unwrap().clone();
        //                 let already_running = animations_running
        //                     .lock()
        //                     .unwrap()
        //                     .clone()
        //                     .into_iter()
        //                     .any(|anim| anim.0 == animation.id());
        //                 if !already_running {
        //                     self.runner.spawn(async move {
        //                         {
        //                             animations_running
        //                                 .lock()
        //                                 .unwrap()
        //                                 .push((animation.id(), tick_message.clone()));
        //                         }

        //                         let mut interval =
        //                             tokio::time::interval(animation.update_interval());
        //                         let (end_value, increment) = match animation.direction() {
        //                             AnimationDirection::Forward => (1., true),
        //                             AnimationDirection::Backward => (0., false),
        //                         };

        //                         while animation.raw_value() != end_value {
        //                             interval.tick().await;
        //                             if increment {
        //                                 animation.increment();
        //                             } else {
        //                                 animation.decrement()
        //                             }
        //                             if proxy.send_event(tick_message.clone()).is_err() {
        //                                 tracing::error!("Failed to send animation tick message")
        //                             }
        //                         }
        //                         {
        //                             animations_running.lock().unwrap().pop();
        //                         }
        //                         if proxy.send_event(done_message.clone()).is_err() {
        //                             tracing::error!("Failed to send animation done message")
        //                         }
        //                     });
        //                 }
        //             }
        //         }
        //     }
        // }
    }

    #[cfg(not(any(android_platform, ios_platform)))]
    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {}
}
