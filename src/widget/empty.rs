use crate::widget::{Constraints, Widget, WidgetCx, WidgetSize};
use parley::FontContext;
use ralaire_core::WidgetId;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Empty<Message> {
    size_hint: WidgetSize,
    phantom_message: PhantomData<Message>,
}

impl<Message> Empty<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    pub fn new(size_hint: WidgetSize) -> Self {
        Self {
            size_hint,
            phantom_message: PhantomData,
        }
    }
    pub fn set_size_hint(&mut self, size_hint: WidgetSize) {
        self.size_hint = size_hint
    }
}

impl<Message> Widget<Message> for Empty<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn children(&self) -> Vec<WidgetId> {
        vec![]
    }

    fn size_hint(&self) -> WidgetSize {
        self.size_hint
    }

    fn layout(
        &mut self,
        _widget_cx: &mut WidgetCx<Message>,
        _constraints: Constraints,
        _font_cx: &mut FontContext,
    ) {
    }
}
