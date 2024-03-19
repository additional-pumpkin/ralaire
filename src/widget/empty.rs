use crate::widget::{Constraints, Widget, WidgetSize};
use parley::FontContext;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct EmptyWidget<Message> {
    size_hint: WidgetSize,
    phantom_message: PhantomData<Message>,
}

impl<Message> EmptyWidget<Message>
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

impl<Message> Widget<Message> for EmptyWidget<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn size_hint(&self) -> WidgetSize {
        self.size_hint
    }

    fn layout(&mut self, _constraints: Constraints, _font_cx: &mut FontContext) {}

    fn children(&self) -> Vec<&super::WidgetData<Message>> {
        vec![]
    }

    fn children_mut(&mut self) -> Vec<&mut super::WidgetData<Message>> {
        vec![]
    }
}
