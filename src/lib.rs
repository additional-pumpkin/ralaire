pub mod app;
pub mod view;
pub mod widget;
use winit::error::EventLoopError;
pub type Error = EventLoopError;

pub type Result = core::result::Result<(), Error>;
