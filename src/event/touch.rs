use super::Point;
use super::Vec2;

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Start { start: Point },
    Move { delta: Vec2 },
    End { end: Point },
    Cancel,
}
