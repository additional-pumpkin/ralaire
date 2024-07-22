use super::WidgetData;
use crate::widget::Widget;
use crate::{
    event::{self, mouse::MouseButton, WidgetEvent},
    InternalMessage,
};
use parley::FontContext;
use vello::kurbo::{Point, Size};
const WINDOW_CONTROLS_WIDTH: f64 = 100.;
const HEADER_HEIGHT: f64 = 46.;

/// like bar but includes window controls (for example minimise, maximise, close)
pub struct Header<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    width: f64,
    left: Option<WidgetData<Message>>,
    middle: Option<WidgetData<Message>>,
    right: Option<WidgetData<Message>>,
    window_controls: WidgetData<Message>,
}

#[allow(dead_code)]
impl<Message> Header<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    pub fn new(
        left: Option<WidgetData<Message>>,
        middle: Option<WidgetData<Message>>,
        right: Option<WidgetData<Message>>,
        window_controls: WidgetData<Message>,
    ) -> Self {
        Self {
            width: 0.,
            left,
            middle,
            right,
            window_controls,
        }
    }
    pub fn left(&mut self) -> &mut Option<WidgetData<Message>> {
        &mut self.left
    }
    pub fn middle(&mut self) -> &mut Option<WidgetData<Message>> {
        &mut self.middle
    }
    pub fn right(&mut self) -> &mut Option<WidgetData<Message>> {
        &mut self.right
    }
}

impl<Message> Widget<Message> for Header<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn debug_name(&self) -> &str {
        "header"
    }
    fn paint(&mut self, scene: &mut vello::Scene) {
        for child in self.children_mut() {
            child.paint(scene);
        }
    }

    fn children(&self) -> Vec<&WidgetData<Message>> {
        self.left
            .iter()
            .chain(self.middle.iter())
            .chain(self.right.iter())
            .chain(std::iter::once(&self.window_controls))
            .collect()
    }

    fn children_mut(&mut self) -> Vec<&mut WidgetData<Message>> {
        self.left
            .iter_mut()
            .chain(self.middle.iter_mut())
            .chain(self.right.iter_mut())
            .chain(std::iter::once(&mut self.window_controls))
            .collect()
    }

    fn layout(&mut self, size_hint: Size, font_cx: &mut FontContext) -> Size {
        if !size_hint.is_finite() {
            tracing::error!("Header widget: size hint is infinite");
        }
        self.width = size_hint.width;
        let side_size_hint = Size {
            width: f64::INFINITY,
            height: HEADER_HEIGHT,
        };
        let left_width;
        let right_width;
        if let Some(left) = &mut self.left {
            left_width = left.layout(side_size_hint, font_cx).width;
        } else {
            left_width = 0.;
        }
        if let Some(right) = &mut self.right {
            right_width = right.layout(side_size_hint, font_cx).width;
        } else {
            right_width = 0.;
        }
        let max_width = f64::max(left_width, right_width + WINDOW_CONTROLS_WIDTH);
        let middle_width = size_hint.width - max_width * 2.;
        if let Some(left) = &mut self.left {
            left.size = Size::new(max_width, HEADER_HEIGHT);
            left.position = Point::new(0., 0.);
        }
        if let Some(right) = &mut self.right {
            right.size = Size::new(max_width, HEADER_HEIGHT);
            right.position = Point::new(max_width + middle_width, 0.);
        }
        let middle_size_hint = Size {
            width: middle_width,
            height: HEADER_HEIGHT,
        };
        // TODO: handle all sizes within constraints
        if let Some(middle) = &mut self.middle {
            middle.layout(middle_size_hint, font_cx);
            middle.size = Size::new(middle_width, HEADER_HEIGHT);
            middle.position = Point::new(max_width, 0.);
        }

        self.window_controls
            .inner
            .layout(Size::new(WINDOW_CONTROLS_WIDTH, HEADER_HEIGHT), font_cx);
        self.window_controls.position = Point::new(self.width - WINDOW_CONTROLS_WIDTH, 0.);
        self.window_controls.size = Size::new(WINDOW_CONTROLS_WIDTH, HEADER_HEIGHT);

        size_hint
    }

    fn event(
        &mut self,
        event: event::WidgetEvent,
        event_cx: &mut event::EventCx<Message>,
    ) -> event::Status {
        if let WidgetEvent::Mouse(event::mouse::Event::Press {
            position: _,
            button: MouseButton::Left,
        }) = event.clone()
        {
            event_cx.push_internal_message(InternalMessage::DragMoveWindow);
            return event::Status::Captured;
        }
        event::Status::Ignored
    }

    fn set_hover(&mut self, _hover: bool) -> event::Status {
        event::Status::Ignored
    }
}
