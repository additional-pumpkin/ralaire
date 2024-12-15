use crate::event::{self, EventContext, WidgetEvent};
use crate::widget::{Widget, WidgetData, WidgetIdPath, WidgetMarker};
use parley::FontContext;
use vello::peniko::kurbo::{Point, Size};

pub struct RootWidget<State> {
    child: WidgetData<State>,
}

impl<State: 'static> RootWidget<State> {
    pub fn new(child: impl Widget<State>) -> Self {
        RootWidget {
            child: WidgetData::new(Box::new(child)),
        }
    }
    pub fn child(&mut self) -> &mut WidgetData<State> {
        &mut self.child
    }

    // TODO: Events should be passed starting at the end of the path
    pub fn send_event(
        &mut self,
        mut event: WidgetEvent,
        event_context: &mut EventContext,
        mut id_path: WidgetIdPath,
        state: &mut State,
    ) {
        // let mut widget_events = Vec::with_capacity(id_path.len());
        self.child.inner.event(event_context, event.clone(), state);
        _ = id_path.remove(0); // skip RootWidget's child
        let mut widget = &mut self.child;
        for id in id_path {
            let child = match widget
                .inner
                .children_mut()
                .into_iter()
                .find(|widget| widget.id == id)
            {
                Some(child) => child,
                None => {
                    tracing::warn!("Tried to send {event:?} to stale widget with id: {id}");
                    return;
                }
            };

            event = event::widget_event(event.clone(), child.position);
            child.inner.event(event_context, event.clone(), state);
            widget = child;
        }
    }
    pub fn send_hover(&mut self, hover: bool, mut id_path: WidgetIdPath) {
        self.child.inner.set_hover(hover);
        _ = id_path.remove(0); // skip RootWidget's child
        let mut widget = &mut self.child;
        for id in id_path {
            let widget_data = match widget
                .inner
                .children_mut()
                .into_iter()
                .find(|widget| widget.id == id)
            {
                Some(widget) => widget,
                None => {
                    tracing::warn!("Tried to send hover={hover} to stale widget with id: {id}");
                    return;
                }
            };
            let child = widget_data;

            child.inner.set_hover(hover);
            widget = child;
        }
    }
}

impl<State> WidgetMarker for RootWidget<State> {}
impl<State: 'static> Widget<State> for RootWidget<State> {
    fn layout(&mut self, suggested_size: Size, font_context: &mut FontContext) -> Size {
        self.child.position = Point::ZERO;
        self.child.size = suggested_size;
        self.child.layout(suggested_size, font_context);
        suggested_size
    }
    fn event(
        &mut self,
        _event_context: &mut event::EventContext,
        _event: event::WidgetEvent,
        _state: &mut State,
    ) -> event::Status {
        event::Status::Ignored
    }
    fn set_hover(&mut self, _hover: bool) -> event::Status {
        event::Status::Ignored
    }
    fn paint(&mut self, scene: &mut vello::Scene) {
        self.child.paint(scene)
    }
    fn children(&self) -> Vec<&WidgetData<State>> {
        vec![&self.child]
    }
    fn children_mut(&mut self) -> Vec<&mut WidgetData<State>> {
        vec![&mut self.child]
    }

    fn debug_name(&self) -> &str {
        "root"
    }
}
