use crate::Size;

pub type WindowId = usize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

impl From<WindowSize> for winit::dpi::PhysicalSize<u32> {
    fn from(value: WindowSize) -> Self {
        winit::dpi::PhysicalSize {
            width: value.width,
            height: value.height,
        }
    }
}

impl From<winit::dpi::PhysicalSize<u32>> for WindowSize {
    fn from(value: winit::dpi::PhysicalSize<u32>) -> Self {
        WindowSize {
            width: value.width,
            height: value.height,
        }
    }
}

impl From<WindowSize> for Size {
    fn from(value: WindowSize) -> Self {
        Size {
            width: value.width as f64,
            height: value.height as f64,
        }
    }
}

impl From<Size> for WindowSize {
    fn from(value: Size) -> Self {
        WindowSize {
            width: value.width as u32,
            height: value.height as u32,
        }
    }
}

impl From<WindowSize> for (u32, u32) {
    fn from(value: WindowSize) -> Self {
        (value.width, value.height)
    }
}

impl From<(u32, u32)> for WindowSize {
    fn from(value: (u32, u32)) -> Self {
        WindowSize {
            width: value.0,
            height: value.1,
        }
    }
}
