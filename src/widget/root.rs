use super::WidgetData;
use crate::widget::Widget;
use crate::{
    event::{self, EventCx, WidgetEvent},
    WidgetIdPath,
};
use parley::FontContext;
use vello::peniko::kurbo::{Point, Size};

pub struct RootWidget<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    child: WidgetData<Message>,
}

impl<Message> RootWidget<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    pub fn new(child: WidgetData<Message>) -> Self {
        RootWidget { child }
    }
    pub fn child(&mut self) -> &mut WidgetData<Message> {
        &mut self.child
    }

    // TODO: Events should be passed starting at the end of the path
    pub fn send_event(
        &mut self,
        mut event: WidgetEvent,
        event_cx: &mut EventCx<Message>,
        mut id_path: WidgetIdPath,
    ) {
        // let mut widget_events = Vec::with_capacity(id_path.len());
        self.child.inner.event(event.clone(), event_cx);
        _ = id_path.remove(0); // skip RootWidget's child
        let mut widget = &mut self.child;
        for id in id_path {
            let child = widget
                .inner
                .children_mut()
                .into_iter()
                .find(|widget| widget.id == id)
                .unwrap_or_else(|| panic!("Stale widget {id:?}"));

            event = event::widget_event(event.clone(), child.position);
            child.inner.event(event.clone(), event_cx);
            widget = child;
        }
    }
    pub fn send_hover(&mut self, hover: bool, mut id_path: WidgetIdPath) {
        self.child.inner.set_hover(hover);
        _ = id_path.remove(0); // skip RootWidget's child
        let mut widget = &mut self.child;
        for id in id_path {
            let child = widget
                .inner
                .children_mut()
                .into_iter()
                .find(|widget| widget.id == id)
                .unwrap_or_else(|| panic!("Stale widget {id:?}"));

            child.inner.set_hover(hover);
            widget = child;
        }
    }
}
impl<Message> Widget<Message> for RootWidget<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn layout(&mut self, size_hint: Size, font_cx: &mut FontContext) -> Size {
        self.child.position = Point::ZERO;
        self.child.size = size_hint;
        self.child.layout(size_hint, font_cx);
        size_hint
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
    fn paint(&mut self, scene: &mut vello::Scene) {
        self.child.paint(scene)
    }
    fn children(&self) -> Vec<&WidgetData<Message>> {
        vec![&self.child]
    }
    fn children_mut(&mut self) -> Vec<&mut WidgetData<Message>> {
        vec![&mut self.child]
    }

    fn debug_name(&self) -> &str {
        "root"
    }
}
