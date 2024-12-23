use std::sync::Arc;
use std::vec::Drain;

use vello::peniko::kurbo::Point;
use vello::peniko::kurbo::Vec2;
use winit::window::CursorIcon;
use winit::window::Window as WinitWindow;
pub mod keyboard;
pub mod mouse;
pub mod touch;
pub mod window;
use crate::InternalMessage;
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Ignored,
    Captured,
}
pub struct EventContext {
    pub repaint_needed: bool,
    pub state_changed: bool,
    pub winit_window: Arc<WinitWindow>,
    internal_messages: Vec<InternalMessage>,
    cursor: CursorIcon,
}
impl EventContext {
    pub fn new(winit_window: Arc<WinitWindow>) -> Self {
        EventContext {
            repaint_needed: false,
            state_changed: false,
            winit_window,
            internal_messages: vec![],
            cursor: CursorIcon::Default,
        }
    }

    pub fn drain_internal_messages(&mut self) -> Drain<InternalMessage> {
        self.internal_messages.drain(..)
    }

    pub fn push_internal_message(&mut self, message: InternalMessage) {
        self.internal_messages.push(message)
    }

    pub fn set_cursor(&mut self, cursor: CursorIcon) {
        self.cursor = cursor;
    }

    pub fn cursor(&self) -> CursorIcon {
        self.cursor
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WidgetEvent {
    Keyboard(keyboard::Event),
    Mouse(mouse::Event),
    Touch(touch::Event),
}

pub fn widget_event_from_window_event(event: window::Event) -> Option<WidgetEvent> {
    match event {
        window::Event::Resized(_) => None,
        window::Event::CloseRequested => None,
        window::Event::ScaleFactorChanged(_) => None,
        window::Event::RedrawRequested => None,
        window::Event::Keyboard(keyboard_event) => Some(WidgetEvent::Keyboard(keyboard_event)),
        window::Event::Mouse(mut mouse_event) => {
            match mouse_event {
                mouse::Event::Move { position } => {
                    mouse_event = mouse::Event::Move { position };
                }
                mouse::Event::Wheel { delta: _ } => {}
                mouse::Event::Press { position, button } => {
                    mouse_event = mouse::Event::Press { position, button };
                }
                mouse::Event::Release { position, button } => {
                    mouse_event = mouse::Event::Release { position, button };
                }
            }
            Some(WidgetEvent::Mouse(mouse_event))
        }
        window::Event::Touch(mut touch_event) => {
            match touch_event {
                touch::Event::Start { start } => {
                    touch_event = touch::Event::Start { start };
                }
                touch::Event::Move { delta: _ } => {}
                touch::Event::End { end } => {
                    touch_event = touch::Event::End { end };
                }
                touch::Event::Cancel => {}
            }
            Some(WidgetEvent::Touch(touch_event))
        }
    }
}

pub fn widget_event(event: WidgetEvent, widget_position: Point) -> WidgetEvent {
    match event {
        WidgetEvent::Keyboard(keyboard_event) => WidgetEvent::Keyboard(keyboard_event),
        WidgetEvent::Mouse(mut mouse_event) => {
            match mouse_event {
                mouse::Event::Move { position } => {
                    mouse_event = mouse::Event::Move {
                        position: (position - widget_position).to_point(),
                    };
                }
                mouse::Event::Wheel { delta: _ } => {}
                mouse::Event::Press { position, button } => {
                    mouse_event = mouse::Event::Press {
                        position: (position - widget_position).to_point(),
                        button,
                    };
                }
                mouse::Event::Release { position, button } => {
                    mouse_event = mouse::Event::Release {
                        position: (position - widget_position).to_point(),
                        button,
                    };
                }
            }
            WidgetEvent::Mouse(mouse_event)
        }
        WidgetEvent::Touch(mut touch_event) => {
            match touch_event {
                touch::Event::Start { start } => {
                    touch_event = touch::Event::Start {
                        start: (start - widget_position).to_point(),
                    };
                }
                touch::Event::Move { delta: _ } => {}
                touch::Event::End { end } => {
                    touch_event = touch::Event::End {
                        end: (end - widget_position).to_point(),
                    };
                }
                touch::Event::Cancel => {}
            }
            WidgetEvent::Touch(touch_event)
        }
    }
}

// Copyright 2019 Héctor Ramón, Iced contributors

/// Converts a winit window event into an ralaire event.
pub fn window_event(
    event: &winit::event::WindowEvent,
    cursor_position: Point,
    scale_factor: f64,
) -> Option<window::Event> {
    use winit::event::WindowEvent;

    match event {
        WindowEvent::RedrawRequested => Some(window::Event::RedrawRequested),
        WindowEvent::Resized(new_size) => Some(window::Event::Resized(*new_size)),
        WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
            Some(window::Event::ScaleFactorChanged(*scale_factor))
        }
        WindowEvent::CloseRequested => Some(window::Event::CloseRequested),
        WindowEvent::CursorMoved { position, .. } => {
            Some(window::Event::Mouse(mouse::Event::Move {
                position: Point::new(position.x / scale_factor, position.y / scale_factor),
            }))
        }
        WindowEvent::MouseInput { button, state, .. } => {
            let button = match button {
                winit::event::MouseButton::Left => Some(mouse::MouseButton::Left),
                winit::event::MouseButton::Right => Some(mouse::MouseButton::Right),
                winit::event::MouseButton::Middle => Some(mouse::MouseButton::Middle),
                winit::event::MouseButton::Back => Some(mouse::MouseButton::Back),
                winit::event::MouseButton::Forward => Some(mouse::MouseButton::Forward),
                _ => None,
            };
            button.map(|button| {
                window::Event::Mouse(match state {
                    winit::event::ElementState::Pressed => mouse::Event::Press {
                        position: cursor_position,
                        button,
                    },
                    winit::event::ElementState::Released => mouse::Event::Release {
                        position: cursor_position,
                        button,
                    },
                })
            })
        }
        WindowEvent::MouseWheel { delta, .. } => match delta {
            winit::event::MouseScrollDelta::LineDelta(delta_x, delta_y) => {
                Some(window::Event::Mouse(mouse::Event::Wheel {
                    delta: Vec2::new(*delta_x as f64 * 30., *delta_y as f64 * 30.),
                }))
            }
            winit::event::MouseScrollDelta::PixelDelta(position) => {
                Some(window::Event::Mouse(mouse::Event::Wheel {
                    delta: Vec2::new(position.x / scale_factor, position.y / scale_factor),
                }))
            }
        },
        _ => None,
    }
}
