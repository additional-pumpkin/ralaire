use super::flex_box::{FlexBox, FlexDirection};
use crate::widget::{Constraints, Length, Widget, WidgetCx, WidgetSize};
use parley::FontContext;
use ralaire_core::WidgetId;

#[derive(Debug)]
pub struct Row<Message> {
    flex: FlexBox<Message>,
}

impl<Message> Row<Message> {
    pub fn new(children: Vec<WidgetId>) -> Self {
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
    Message: core::fmt::Debug + Clone + 'static,
{
    fn children(&self) -> Vec<WidgetId> {
        self.flex.children()
    }

    fn size_hint(&self) -> WidgetSize {
        WidgetSize {
            width: Length::Flexible(1),
            height: Length::Flexible(1),
        }
    }
    fn layout(
        &mut self,
        widget_cx: &mut WidgetCx<Message>,
        constraints: Constraints,
        font_cx: &mut FontContext,
    ) {
        self.flex.layout(widget_cx, constraints, font_cx);
    }
}
