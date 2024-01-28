use super::keyboard;
use super::mouse;
use super::touch;
use crate::WindowSize;
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    WindowResize(WindowSize),
    WindowCloseRequested,
    WindowScaleFactorChanged(f64),
    WindowRedrawRequested,
    Keyboard(keyboard::Event),
    Mouse(mouse::Event),
    Touch(touch::Event),
}
