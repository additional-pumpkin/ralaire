use super::{button::ButtonView, container::ContainerView, FlexBoxView, View, ViewSequence};

#[macro_export]
macro_rules! view_seq {
    ($($x:expr),+ $(,)?) => {$crate::view::ViewSequence(
        vec![$(Box::new($x)),+],
    )};
}

pub fn flex<Message>(children: ViewSequence<Message>) -> FlexBoxView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    FlexBoxView::new(children)
}

pub fn button<Message>(child: impl View<Message>) -> ButtonView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    ButtonView::new(Box::new(child))
}
pub fn container<Message>(child: impl View<Message>) -> ContainerView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    ContainerView::new(Box::new(child))
}
