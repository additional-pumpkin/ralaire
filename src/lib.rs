pub mod alignment;
mod animation;
mod any;
pub mod app;
mod command;
mod debug;
mod event;
mod id;
mod padding;
mod renderer;
mod window;
pub use animation::Animation;
pub use animation::AnimationDirection;
pub use animation::EasingCurve;
use any::AsAny;
use app::InternalMessage;
pub use command::Command;
use debug::DebugLayer;
use id::AnimationId;
use id::WidgetId;
use id::WidgetIdPath;
use padding::Padding;
pub use window::WindowId;
use window::WindowSize;
pub mod view;
pub mod widget;
use winit::error::EventLoopError;
pub type Error = EventLoopError;

pub type Result = core::result::Result<(), Error>;
