use std::marker::PhantomData;

use crate::renderer::PaintCx;
use crate::widget::ContainerWidget;
use crate::widget::{Constraints, Widget};
use crate::InternalMessage;
use crate::{
    alignment,
    event::{self, mouse::MouseButton},
};
use parley::FontContext;
use peniko::kurbo::{Circle, Point, Size};
use peniko::Color;

use super::WidgetData;

const SIZE: Size = Size::new(24., 24.);
const CENTER: Point = Point::new(SIZE.width / 2., SIZE.height / 2.);
const RADIUS: f64 = SIZE.width / 2.;
pub struct WindowButtonWidget<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    on_press: InternalMessage,
    child: ContainerWidget<Message>,
    hovered: bool,
    phantom_message: PhantomData<Message>,
}

impl<Message> WindowButtonWidget<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    pub fn new(child: WidgetData<Message>, on_press: InternalMessage) -> Self {
        let child = ContainerWidget::new(
            child,
            alignment::Horizontal::Center,
            alignment::Vertical::Center,
            0.0.into(),
        );
        Self {
            on_press,
            child,
            hovered: false,
            phantom_message: PhantomData,
        }
    }
}

impl<Message> Widget<Message> for WindowButtonWidget<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn debug_name(&self) -> &str {
        "window_button"
    }
    fn paint(&self, paint_cx: &mut PaintCx) {
        paint_cx.fill_shape(&Circle::new(CENTER, RADIUS), Color::LIGHT_GRAY);
        if self.hovered {
            paint_cx.fill_shape(
                &Circle::new(CENTER, RADIUS),
                Color::BLACK.with_alpha_factor(0.1),
            );
        }
    }

    fn children(&self) -> Vec<&WidgetData<Message>> {
        self.child.children()
    }

    fn children_mut(&mut self) -> Vec<&mut WidgetData<Message>> {
        self.child.children_mut()
    }

    fn layout(&mut self, _constraints: Constraints, font_cx: &mut FontContext) -> Size {
        self.child.layout(
            Constraints {
                min_size: SIZE,
                max_size: SIZE,
            },
            font_cx,
        );
        SIZE
    }

    fn event(
        &mut self,
        event: event::WidgetEvent,
        event_cx: &mut event::EventCx<Message>,
    ) -> event::Status {
        if let event::WidgetEvent::Mouse(event::mouse::Event::Press {
            position: _,
            button,
        }) = event
        {
            if button == MouseButton::Left {
                event_cx.push_internal_message(self.on_press.clone());
                return event::Status::Captured;
            }
        }
        event::Status::Ignored
    }
    fn set_hover(&mut self, hover: bool) -> event::Status {
        self.hovered = hover;
        tracing::debug!("Set hovered to {}", hover);
        event::Status::Captured
    }
}
