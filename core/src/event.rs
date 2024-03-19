use peniko::kurbo::Point;
use peniko::kurbo::Vec2;
pub mod keyboard;
pub mod mouse;
pub mod touch;
pub mod window;
use crate::WindowId;
use winit::window::CursorIcon;
extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum Cursor {
    Default,
    Pointer,
    EResize,
    NResize,
    NeResize,
    NwResize,
    SResize,
    SeResize,
    SwResize,
    WResize,
}
#[derive(Debug, Clone, Copy)]
pub enum ResizeDirection {
    East,
    North,
    NorthEast,
    NorthWest,
    South,
    SouthEast,
    SouthWest,
    West,
}

impl From<ResizeDirection> for winit::window::ResizeDirection {
    fn from(value: ResizeDirection) -> Self {
        match value {
            ResizeDirection::East => winit::window::ResizeDirection::East,
            ResizeDirection::North => winit::window::ResizeDirection::North,
            ResizeDirection::NorthEast => winit::window::ResizeDirection::NorthEast,
            ResizeDirection::NorthWest => winit::window::ResizeDirection::NorthWest,
            ResizeDirection::South => winit::window::ResizeDirection::South,
            ResizeDirection::SouthEast => winit::window::ResizeDirection::SouthEast,
            ResizeDirection::SouthWest => winit::window::ResizeDirection::SouthWest,
            ResizeDirection::West => winit::window::ResizeDirection::West,
        }
    }
}

impl From<Cursor> for CursorIcon {
    fn from(value: Cursor) -> Self {
        match value {
            Cursor::Default => CursorIcon::Default,
            Cursor::Pointer => CursorIcon::Pointer,
            Cursor::EResize => CursorIcon::EResize,
            Cursor::NResize => CursorIcon::NResize,
            Cursor::NeResize => CursorIcon::NeResize,
            Cursor::NwResize => CursorIcon::NwResize,
            Cursor::SResize => CursorIcon::SResize,
            Cursor::SeResize => CursorIcon::SeResize,
            Cursor::SwResize => CursorIcon::SwResize,
            Cursor::WResize => CursorIcon::WResize,
        }
    }
}

impl From<CursorIcon> for Cursor {
    fn from(value: CursorIcon) -> Self {
        match value {
            CursorIcon::Default => Cursor::Default,
            CursorIcon::Pointer => Cursor::Pointer,
            CursorIcon::EResize => Cursor::EResize,
            CursorIcon::NResize => Cursor::NResize,
            CursorIcon::NeResize => Cursor::NeResize,
            CursorIcon::NwResize => Cursor::NwResize,
            CursorIcon::SResize => Cursor::SResize,
            CursorIcon::SeResize => Cursor::SeResize,
            CursorIcon::SwResize => Cursor::SwResize,
            CursorIcon::WResize => Cursor::WResize,
            _ => Cursor::Default,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Window {
        window_id: WindowId,
        event: window::Event,
    },
    // UserEvent(T),
}

#[derive(Debug, PartialEq)]
pub enum Status {
    Ignored,
    Captured,
}
pub struct EventCx<Message> {
    messages: Vec<Message>,
    cursor: Cursor,
}
impl<Message> EventCx<Message> {
    pub fn new() -> Self {
        EventCx {
            messages: vec![],
            cursor: Cursor::Default,
        }
    }

    pub fn messages(&mut self) -> Vec<Message> {
        self.messages.drain(..).collect()
    }
    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message)
    }

    pub fn set_cursor(&mut self, cursor: Cursor) {
        self.cursor = cursor;
    }

    pub fn cursor(&self) -> Cursor {
        self.cursor
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WidgetEvent {
    Keyboard(keyboard::Event),
    Mouse(mouse::Event),
    Touch(touch::Event),
}

pub fn widget_event_from_window_event(
    event: window::Event,
    widget_position: Point,
) -> Option<WidgetEvent> {
    match event {
        window::Event::Resized(_) => None,
        window::Event::CloseRequested => None,
        window::Event::ScaleFactorChanged(_) => None,
        window::Event::RedrawRequested => None,
        window::Event::Keyboard(keyboard_event) => Some(WidgetEvent::Keyboard(keyboard_event)),
        window::Event::Mouse(mut mouse_event) => {
            match mouse_event {
                mouse::Event::Move { position, delta } => {
                    mouse_event = mouse::Event::Move {
                        position: (position - widget_position).to_point(),
                        delta,
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
            Some(WidgetEvent::Mouse(mouse_event))
        }
        window::Event::Touch(mut touch_event) => {
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
            Some(WidgetEvent::Touch(touch_event))
        }
    }
}

pub fn widget_event(event: WidgetEvent, widget_position: Point) -> WidgetEvent {
    match event {
        WidgetEvent::Keyboard(keyboard_event) => WidgetEvent::Keyboard(keyboard_event),
        WidgetEvent::Mouse(mut mouse_event) => {
            match mouse_event {
                mouse::Event::Move { position, delta } => {
                    mouse_event = mouse::Event::Move {
                        position: (position - widget_position).to_point(),
                        delta,
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
) -> Option<window::Event> {
    use winit::event::WindowEvent;

    match event {
        WindowEvent::RedrawRequested => Some(window::Event::RedrawRequested),
        WindowEvent::Resized(new_size) => Some(window::Event::Resized((*new_size).into())),
        WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
            Some(window::Event::ScaleFactorChanged(*scale_factor))
        }
        WindowEvent::CloseRequested => Some(window::Event::CloseRequested),
        WindowEvent::CursorMoved { position, .. } => {
            Some(window::Event::Mouse(mouse::Event::Move {
                position: Point::new(position.x, position.y),
                delta: Point::new(-position.x, -position.y) - cursor_position,
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
            if let Some(button) = button {
                Some(window::Event::Mouse(match state {
                    winit::event::ElementState::Pressed => mouse::Event::Press {
                        position: cursor_position,
                        button,
                    },
                    winit::event::ElementState::Released => mouse::Event::Release {
                        position: cursor_position,
                        button,
                    },
                }))
            } else {
                None
            }
        }
        WindowEvent::MouseWheel { delta, .. } => match delta {
            winit::event::MouseScrollDelta::LineDelta(delta_x, delta_y) => {
                Some(window::Event::Mouse(mouse::Event::Wheel {
                    delta: Vec2::new(*delta_x as f64, *delta_y as f64),
                }))
            }
            winit::event::MouseScrollDelta::PixelDelta(position) => {
                Some(window::Event::Mouse(mouse::Event::Wheel {
                    delta: Vec2::new(position.x as f64, position.y as f64),
                }))
            }
        },
        _ => None,
    }
}
