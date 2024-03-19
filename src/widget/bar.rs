use std::marker::PhantomData;

use crate::widget::{Constraints, Length, Widget, WidgetCx, WidgetSize};
use parley::FontContext;
#[allow(unused_imports)]
use ralaire_core::{Point, RenderCx, Size, WidgetId};

#[allow(dead_code)]
const BAR_HEIGHT: f64 = 32.;

#[derive(Debug)]
pub struct Bar<Message> {
    children: [WidgetId; 3],
    phantom_data: PhantomData<Message>,
}

impl<Message> Bar<Message> {
    #[allow(dead_code)]
    pub fn new(children: [WidgetId; 3]) -> Self {
        Self {
            children,
            phantom_data: PhantomData,
        }
    }
}

impl<Message> Widget<Message> for Bar<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn draw(&self, _render_cx: &mut RenderCx) {}

    fn children(&self) -> Vec<WidgetId> {
        self.children.iter().map(|id| id.clone()).collect()
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
        let left = self.children[0];
        let middle = self.children[1];
        let right = self.children[2];
        widget_cx.layout(left, constraints, font_cx);
        widget_cx.layout(middle, constraints, font_cx);
        widget_cx.layout(right, constraints, font_cx);
        let _left_width = widget_cx.size_hint(left).width;
        let _middle_width = widget_cx.size_hint(middle).width;
        let _right_width = widget_cx.size_hint(right).width;
    }
}
