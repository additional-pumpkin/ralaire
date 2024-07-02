use std::path::PathBuf;

use super::{
    button::ButtonView, container::ContainerView, header::HeaderView, slider::SliderView,
    window::WindowView, FlexBoxView, ImageView, View, ViewSequence,
};

#[macro_export]
macro_rules! view_seq {
    ($($x:expr),+ $(,)?) => {$crate::view::ViewSequence(
        vec![$(Box::new($x)),+],
    )};
}

pub fn window<Message>(child: impl View<Message>) -> WindowView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    WindowView::new(Box::new(HeaderView::new()), Box::new(child))
}

pub fn flex<Message>(children: ViewSequence<Message>) -> FlexBoxView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    let len = children.0.len();
    FlexBoxView::new(children, vec![None; len])
}

pub fn button<Message>(child: impl View<Message>) -> ButtonView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    ButtonView::new(Box::new(child))
}
pub fn slider<Message>(value: f64, on_change: fn(f64) -> Message) -> SliderView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    SliderView::new(value, on_change)
}

pub fn container<Message>(child: impl View<Message>) -> ContainerView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    ContainerView::new(Box::new(child))
}
pub fn image(path: PathBuf) -> ImageView {
    ImageView::new(path)
}
