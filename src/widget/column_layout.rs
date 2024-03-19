use parley::FontContext;
use ralaire_core::WidgetId;

use super::flex_box::{FlexBox, FlexDirection};
use crate::widget::{Constraints, Length, Widget, WidgetCx, WidgetSize};

#[derive(Debug)]
pub struct Column<Message> {
    flex: FlexBox<Message>,
}

impl<Message> Column<Message> {
    pub fn new(children: Vec<WidgetId>) -> Self {
        Column {
            flex: FlexBox::new(children, FlexDirection::Column),
        }
    }
    pub fn spacing(mut self, spacing: f64) -> Self {
        self.flex = self.flex.with_spacing(spacing);
        self
    }
}

impl<Message> Widget<Message> for Column<Message>
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
