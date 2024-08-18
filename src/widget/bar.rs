use std::marker::PhantomData;

use crate::event;
use crate::widget::Widget;
use parley::FontContext;
use vello::kurbo::{Point, Size};

use super::WidgetData;

pub struct Bar<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    left: Option<WidgetData<Message>>,
    middle: Option<WidgetData<Message>>,
    right: Option<WidgetData<Message>>,
    height: f64,
    phantom_data: PhantomData<Message>,
}

impl<Message> Bar<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    pub fn new(
        left: Option<WidgetData<Message>>,
        middle: Option<WidgetData<Message>>,
        right: Option<WidgetData<Message>>,
        height: f64,
    ) -> Self {
        Self {
            left,
            middle,
            right,
            height,
            phantom_data: PhantomData,
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
    pub fn set_height(&mut self, height: f64) {
        self.height = height
    }
}

impl<Message> Widget<Message> for Bar<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn debug_name(&self) -> &str {
        "bar"
    }
    fn paint(&mut self, scene: &mut vello::Scene) {
        for child in self.children_mut() {
            child.paint(scene);
        }
    }

    fn layout(&mut self, size_hint: Size, font_cx: &mut FontContext) -> Size {
        if !size_hint.is_finite() {
            tracing::error!("Bar widget: suggested size is infinite");
        }
        let side_size_hint = Size {
            width: f64::INFINITY,
            height: self.height,
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
        let max_width = f64::max(left_width, right_width);
        let middle_width = size_hint.width - max_width * 2.;
        if let Some(left) = &mut self.left {
            left.size = Size::new(max_width, self.height);
            left.position = Point::new(0., 0.);
        }
        if let Some(right) = &mut self.right {
            right.size = Size::new(max_width, self.height);
            right.position = Point::new(max_width + middle_width, 0.);
        }
        let middle_size_hint = Size {
            width: middle_width,
            height: self.height,
        };
        if let Some(middle) = &mut self.middle {
            middle.layout(middle_size_hint, font_cx);
            middle.size = Size::new(middle_width, self.height);
            middle.position = Point::new(max_width, 0.);
        }
        Size::new(size_hint.width, self.height)
    }

    fn children(&self) -> Vec<&WidgetData<Message>> {
        self.left
            .iter()
            .chain(self.middle.iter())
            .chain(self.right.iter())
            .collect()
    }

    fn children_mut(&mut self) -> Vec<&mut WidgetData<Message>> {
        self.left
            .iter_mut()
            .chain(self.middle.iter_mut())
            .chain(self.right.iter_mut())
            .collect()
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
