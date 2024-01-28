use crate::widget::{Button, Container, Text, Widget};

pub fn button<Message>(child: impl Widget<Message> + 'static) -> Button<Message>
where
    Message: std::fmt::Debug + Clone + 'static,
{
    Button::new(child)
}

pub fn text(text: impl Into<String>) -> Text {
    Text::new(text.into())
}

pub fn container<Message>(child: impl Widget<Message> + 'static) -> Container<Message>
where
    Message: std::fmt::Debug + Clone,
{
    Container::new(child)
}

pub fn empty() -> Empty {
    Empty::new()
}

#[macro_export]
macro_rules! row {
    () => {$crate::Row::new()};
    ($($x:expr),+ $(,)?) => {$crate::widget::Row::new(vec![$($crate::widget::WidgetData::new($x)),+])};
}

pub use row;

#[macro_export]
macro_rules! column {
    () => {$crate::Column::new()};
    ($($x:expr),+ $(,)?) => {$crate::widget::Column::new(vec![$($crate::widget::WidgetData::new($x)),+])};
}

pub use column;

use super::Empty;
