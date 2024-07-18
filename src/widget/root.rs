use super::WidgetData;
use crate::widget::{Constraints, Widget};
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
        self.child.widget.event(event.clone(), event_cx);
        _ = id_path.remove(0); // skip RootWidget's child
        let mut widget = &mut self.child;
        for id in id_path {
            let child = widget
                .widget
                .children_mut()
                .into_iter()
                .find(|widget| widget.id == id)
                .unwrap_or_else(|| panic!("Stale widget {id:?}"));

            event = event::widget_event(event.clone(), child.position);
            child.widget.event(event.clone(), event_cx);
            widget = child;
        }
    }
    pub fn send_hover(&mut self, hover: bool, mut id_path: WidgetIdPath) {
        self.child.widget.set_hover(hover);
        _ = id_path.remove(0); // skip RootWidget's child
        let mut widget = &mut self.child;
        for id in id_path {
            let child = widget
                .widget
                .children_mut()
                .into_iter()
                .find(|widget| widget.id == id)
                .unwrap_or_else(|| panic!("Stale widget {id:?}"));

            child.widget.set_hover(hover);
            widget = child;
        }
    }
}
impl<Message> Widget<Message> for RootWidget<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn layout(&mut self, constraints: Constraints, font_cx: &mut FontContext) -> Size {
        let size = constraints.max_size;
        self.child.widget.layout(
            Constraints {
                min_size: size,
                max_size: size,
            },
            font_cx,
        );
        self.child.position = Point::ZERO;
        self.child.size = size;
        size
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
    fn paint(&self, scene: &mut vello::Scene) {
        self.child.widget.paint(scene)
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
