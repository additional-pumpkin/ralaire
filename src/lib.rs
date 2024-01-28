pub mod app;
pub mod widget;
use winit::error::EventLoopError;
pub type Error = EventLoopError;

pub type Result = std::result::Result<(), Error>;
