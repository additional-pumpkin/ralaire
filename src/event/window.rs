use super::keyboard;
use super::mouse;
use super::touch;
use crate::WindowSize;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Resized(WindowSize),
    CloseRequested,
    ScaleFactorChanged(f64),
    RedrawRequested,
    Keyboard(keyboard::Event),
    Mouse(mouse::Event),
    Touch(touch::Event),
}
