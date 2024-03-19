use super::App;
use crate::{app::window::AppWindow, view::RootView};
use parley::FontContext;
use ralaire_core::{AnimationId, DebugLayer};
use std::sync::{Arc, Mutex};
use winit::{
    error::EventLoopError,
    event::{Event, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
};

pub struct AppState<'a, Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    pub runner: tokio::runtime::Runtime,
    pub debug: DebugLayer,
    pub event_loop: EventLoop<Message>,
    pub title: String,
    font_cx: FontContext,
    messages: Vec<Message>,
    animations_running: Arc<Mutex<Vec<(AnimationId, Message)>>>,
    main_window: Option<AppWindow<'a, Message>>,
    _child_windows: Vec<AppWindow<'a, Message>>,
}
impl<'a, Message> Default for AppState<'a, Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn default() -> Self {
        let runner = tokio::runtime::Builder::new_multi_thread()
            .enable_time()
            .build()
            .unwrap();
        AppState {
            runner,
            debug: DebugLayer::new(),
            event_loop: EventLoop::with_user_event().build().unwrap(),
            font_cx: FontContext::new(),
            title: String::from(""),
            messages: vec![],
            animations_running: Arc::new(Mutex::new(vec![])),
            main_window: None,
            _child_windows: vec![],
        }
    }
}
impl<'a, Message> AppState<'a, Message>
where
    Message: core::fmt::Debug + Clone + Send,
{
    pub fn new() -> Self {
        AppState::default()
    }

    fn run_event_loop<F, T>(
        event_loop: EventLoop<T>,
        event_handler: F,
    ) -> Result<(), EventLoopError>
    where
        F: FnMut(Event<T>, &ActiveEventLoop),
    {
        event_loop.run(event_handler)
    }
    pub fn run<A>(mut self) -> Result<(), EventLoopError>
    where
        A: App,
        A: App<Message = Message>,
    {
        let mut app = A::new();
        let mut new_size = winit::dpi::PhysicalSize::<u32>::default();
        let event_loop = self.event_loop;
        let event_loop_proxy = Arc::new(event_loop.create_proxy());

        let event_handler = move |event: Event<Message>, event_loop: &_| match event {
            Event::Resumed => {
                let main_window = self.runner.block_on(AppWindow::new(
                    event_loop,
                    app.title().into(),
                    RootView::new(Box::new(app.view())),
                    &mut self.debug,
                ));
                self.main_window = Some(main_window);
                if let Some(main_window) = &mut self.main_window {
                    new_size = main_window.size();
                }
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } => {
                if let Some(main_window) = &mut self.main_window {
                    if window_id == main_window.id() {
                        match event {
                            WindowEvent::CloseRequested => {
                                event_loop.exit();
                            }
                            WindowEvent::Resized(physical_size) => {
                                new_size = *physical_size;
                                main_window.resize(new_size, &mut self.font_cx, &mut self.debug);
                            }

                            WindowEvent::RedrawRequested => {
                                self.debug.draw_started();
                                main_window.paint();
                                self.debug.draw_finished();

                                self.debug.render_started();
                                self.runner.block_on(main_window.render(&mut self.debug));
                                self.debug.render_finished();
                                self.debug.log();
                            }
                            _ => {
                                self.debug.event_started();
                                let event = ralaire_core::event::window_event(
                                    event,
                                    main_window.cursor_pos(),
                                );
                                if let Some(event) = event {
                                    let messages = main_window.event(event);
                                    let proxy = event_loop_proxy.clone();
                                    for message in messages {
                                        if proxy.send_event(message.clone()).is_err() {
                                            tracing::error!("Failed to send Message: {:?}", message)
                                        }
                                    }
                                    self.debug.event_finished();
                                }
                            }
                        }
                    }
                }
            }
            Event::AboutToWait => {
                self.debug.update_started();
                let mut commands = vec![];
                let mut updated = false;
                if !self.messages.is_empty() {
                    updated = true;
                }
                for message in self.messages.drain(..) {
                    commands.extend(app.update(message));
                }
                self.debug.update_finished();

                if let Some(main_window) = &mut self.main_window {
                    // TODO: reconciliate view trees here
                    main_window.set_title(app.title().into());
                    self.debug.view_started();
                    let new = RootView::new(Box::new(app.view()));
                    new.reconciliate(
                        &main_window.root_view,
                        &mut main_window.root_widget,
                        &mut main_window.widget_cx,
                    );
                    main_window.set_root_view(new);
                    self.debug.view_finished();

                    // NOTE: This is to force doing layout since the widgets are new
                    // TODO: Make set_title() and set_header() set a needs layout flag
                    main_window.resize(new_size, &mut self.font_cx, &mut self.debug);
                    if updated {
                        main_window.update();
                        main_window.request_redraw();
                    }
                }
                for command in commands {
                    match command {
                        ralaire_core::Command::Animate {
                            mut animation,
                            tick_message,
                            done_message,
                        } => {
                            let animations_running = self.animations_running.clone();
                            let proxy = event_loop_proxy.clone();
                            let already_running = animations_running
                                .lock()
                                .unwrap()
                                .clone()
                                .into_iter()
                                .find(|anim| anim.0 == animation.id())
                                .is_some();
                            if !already_running {
                                self.runner.spawn(async move {
                                    {
                                        animations_running
                                            .lock()
                                            .unwrap()
                                            .push((animation.id().clone(), tick_message.clone()));
                                    }

                                    let mut interval =
                                        tokio::time::interval(animation.update_interval());
                                    let (end_value, increment) = match animation.direction() {
                                        ralaire_core::AnimationDirection::Forward => (1., true),
                                        ralaire_core::AnimationDirection::Backward => (0., false),
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
                    };
                }
            }
            Event::UserEvent(message) => {
                let animations_running = self.animations_running.clone();
                let animations_running = animations_running.lock().unwrap();
                if animations_running
                    .clone()
                    .into_iter()
                    .find(|anim| {
                        core::mem::discriminant(&anim.1) == std::mem::discriminant(&message)
                    })
                    .is_some()
                {
                    app.update(message);
                    if let Some(main_window) = &mut self.main_window {
                        main_window.update();
                        main_window.request_redraw();
                    }
                } else {
                    self.messages.push(message);
                }
            }
            _ => {}
        };
        AppState::<Message>::run_event_loop(event_loop, event_handler)
    }
}
