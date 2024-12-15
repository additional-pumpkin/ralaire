mod animation;
pub mod app;
mod event;
mod padding;
mod renderer;
mod scene;
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

use app::InternalMessage;
use padding::Padding;
pub use task::Task;
pub mod view;
pub mod widget;
