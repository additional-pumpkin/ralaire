use crate::widget::Widget;
use parley::FontContext;

use super::{
    flex_layout::{FlexBox, FlexDirection},
    widget::{Constraints, Length, WidgetData, WidgetSize},
};

#[derive(Debug)]
pub struct Row<Message> {
    flex: FlexBox<Message>,
}

impl<Message> Row<Message>
where
    Message: Clone,
{
    pub fn new(children: Vec<WidgetData<Message>>) -> Self {
        Row {
            flex: FlexBox::new(children, FlexDirection::Row),
        }
    }
    pub fn spacing(mut self, spacing: f64) -> Self {
        self.flex = self.flex.with_spacing(spacing);
        self
    }
}

impl<Message> Widget<Message> for Row<Message>
where
    Message: std::fmt::Debug + Clone,
{
    fn children(&self) -> Vec<&WidgetData<Message>> {
        self.flex.children()
    }

    fn children_mut(&mut self) -> Vec<&mut WidgetData<Message>> {
        self.flex.children_mut()
    }

    fn size_hint(&self) -> WidgetSize {
        WidgetSize {
            width: Length::Flexible(1),
            height: Length::Flexible(1),
        }
    }

    fn layout(&mut self, constraints: Constraints, font_cx: &mut FontContext) {
        self.flex.layout(constraints, font_cx);
    }
}
