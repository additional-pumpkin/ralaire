use crate::event;
use crate::renderer::PaintCx;
use crate::widget::{Constraints, Widget};
use parley::FontContext;
use peniko::kurbo::Size;
use std::marker::PhantomData;
pub struct EmptyWidget<Message> {
    size: Size,
    phantom_message: PhantomData<Message>,
}

impl<Message> EmptyWidget<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    pub fn new(size: Size) -> Self {
        Self {
            size,
            phantom_message: PhantomData,
        }
    }
    pub fn set_size(&mut self, size: Size) {
        self.size = size
    }
}

impl<Message> Widget<Message> for EmptyWidget<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn paint(&self, _paint_cx: &mut PaintCx) {}

    fn debug_name(&self) -> &str {
        "empty"
    }
    fn layout(&mut self, _constraints: Constraints, _font_cx: &mut FontContext) -> Size {
        self.size
    }

    fn children(&self) -> Vec<&super::WidgetData<Message>> {
        vec![]
    }

    fn children_mut(&mut self) -> Vec<&mut super::WidgetData<Message>> {
        vec![]
    }

    fn event(
        &mut self,
        _event: event::WidgetEvent,
        _event_cx: &mut event::EventCx<Message>,
    ) -> event::Status {
        event::Status::Ignored
    }

    fn set_hover(&mut self, _hover: bool) -> event::Status {
        event::Status::Ignored
    }
}
