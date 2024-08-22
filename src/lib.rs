mod animation;
pub mod application;
mod debug;
mod event;
mod id;
mod padding;
mod renderer;
mod task;
mod window;
pub use animation::Animation;
pub use animation::AnimationDirection;
pub use animation::EasingCurve;
use core::any::Any;
pub trait AsAny: 'static {
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_any(&self) -> &dyn Any;
    fn type_name(&self) -> &'static str;
}

impl<T: 'static> AsAny for T {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}

use animation::AnimationId;
use application::InternalMessage;
use debug::DebugLayer;
use padding::Padding;
pub use task::Task;
pub mod view;
pub mod widget;
