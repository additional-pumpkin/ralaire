use super::keyboard;
use super::mouse;
use super::touch;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Resized(winit::dpi::PhysicalSize<u32>),
    CloseRequested,
    ScaleFactorChanged(f64),
    RedrawRequested,
    Keyboard(keyboard::Event),
    Mouse(mouse::Event),
    Touch(touch::Event),
}
