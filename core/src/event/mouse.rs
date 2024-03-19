use super::Point;
use super::Vec2;

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Move {
        position: Point,
        delta: Vec2,
    },
    Wheel {
        delta: Vec2,
    },
    Press {
        position: Point,
        button: MouseButton,
    },
    Release {
        position: Point,
        button: MouseButton,
    },
}
#[derive(Debug, Clone, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
}
