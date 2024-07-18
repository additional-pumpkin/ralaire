use super::App;
use crate::command::Task;
use crate::{app::window::AppWindow, view::RootView};
use crate::{event::window_event, AnimationId, DebugLayer};
use crate::{AnimationDirection, Command};
use parley::FontContext;
use std::sync::{Arc, Mutex};
use winit::{
    application::ApplicationHandler,
    error::EventLoopError,
    event::{DeviceEvent, DeviceId, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    window::WindowId,
};

pub struct AppState<'a, A: App> {
    pub app: A,
    pub event_loop_proxy: Option<EventLoopProxy<A::Message>>,
    pub runner: tokio::runtime::Runtime,
    pub debug: DebugLayer,
    font_cx: FontContext,
    messages: Vec<A::Message>,
    animations_running: Arc<Mutex<Vec<(AnimationId, A::Message)>>>,
    main_window: Option<AppWindow<'a, A::Message>>,
    _child_windows: Vec<AppWindow<'a, A::Message>>,
}
impl<'a, A> Default for AppState<'a, A>
where
    A: App,
{
    fn default() -> Self {
        let runner = tokio::runtime::Builder::new_multi_thread()
            .enable_time()
            .build()
            .unwrap();
        AppState {
            app: App::new(),
            event_loop_proxy: None,
            runner,
            debug: DebugLayer::new(),
            font_cx: FontContext::default(),
            messages: vec![],
            animations_running: Arc::new(Mutex::new(vec![])),
            main_window: None,
            _child_windows: vec![],
        }
    }
}
impl<'a, A> AppState<'a, A>
where
    A: App,
{
    pub fn new() -> Self {
        AppState::default()
    }

    pub fn run(mut self) -> Result<(), EventLoopError> {
        let event_loop: EventLoop<A::Message> = EventLoop::with_user_event().build().unwrap();
        let event_loop_proxy = event_loop.create_proxy();
        self.event_loop_proxy = Some(event_loop_proxy);
        event_loop.run_app(&mut self)
    }
}

impl<'a, A: App> ApplicationHandler<A::Message> for AppState<'a, A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        tracing::trace!("Resumed the event loop");
        let main_window = self.runner.block_on(AppWindow::new(
            event_loop,
            "TODO: Make titles work?".to_owned(),
            RootView::new(Box::new(self.app.view())),
            &mut self.debug,
        ));
        self.main_window = Some(main_window);
    }
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, message: A::Message) {
        tracing::trace!("User message: {message:?}");
        self.messages.push(message);
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
        let messages = window.event(event, event_loop, &mut self.font_cx, &mut self.debug);
        let proxy = self.event_loop_proxy.as_ref().unwrap().clone();
        for message in messages {
            if proxy.send_event(message.clone()).is_err() {
                tracing::error!("Failed to send Message: {:?}", message)
            }
        }
        self.debug.event_finished();
    }
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if self.messages.is_empty() {
            return;
        }
        tracing::trace!("messages: {:?}", self.messages.clone());
        let mut commands = vec![];
        self.debug.update_started();
        for message in self.messages.drain(..) {
            commands.push(self.app.update(message));
        }
        self.debug.update_finished();

        if let Some(main_window) = &mut self.main_window {
            // main_window.set_title();
            self.debug.view_started();
            let new = RootView::new(Box::new(self.app.view()));
            new.reconciliate(&main_window.root_view, &mut main_window.root_widget);
            main_window.set_root_view(new);
            self.debug.view_finished();
            self.debug.layout_started();
            main_window.layout(&mut self.font_cx);
            self.debug.layout_finished();
            main_window.request_redraw();
        }
        for command in commands {
            for task in command.tasks {
                match task {
                    Task::Animate {
                        mut animation,
                        tick_message,
                        done_message,
                    } => {
                        let animations_running = self.animations_running.clone();
                        let proxy = self.event_loop_proxy.as_ref().unwrap().clone();
                        let already_running = animations_running
                            .lock()
                            .unwrap()
                            .clone()
                            .into_iter()
                            .any(|anim| anim.0 == animation.id());
                        if !already_running {
                            self.runner.spawn(async move {
                                {
                                    animations_running
                                        .lock()
                                        .unwrap()
                                        .push((animation.id(), tick_message.clone()));
                                }

                                let mut interval =
                                    tokio::time::interval(animation.update_interval());
                                let (end_value, increment) = match animation.direction() {
                                    AnimationDirection::Forward => (1., true),
                                    AnimationDirection::Backward => (0., false),
                                };

                                while animation.raw_value() != end_value {
                                    interval.tick().await;
                                    if increment {
                                        animation.increment();
                                    } else {
                                        animation.decrement()
                                    }
                                    if proxy.send_event(tick_message.clone()).is_err() {
                                        tracing::error!("Failed to send animation tick message")
                                    }
                                }
                                {
                                    animations_running.lock().unwrap().pop();
                                }
                                if proxy.send_event(done_message.clone()).is_err() {
                                    tracing::error!("Failed to send animation done message")
                                }
                            });
                        }
                    }
                }
            }
        }
    }

    #[cfg(not(any(android_platform, ios_platform)))]
    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {}
}
