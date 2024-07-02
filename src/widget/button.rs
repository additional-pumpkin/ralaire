use crate::renderer::PaintCx;
use crate::widget::ContainerWidget;
use crate::widget::{Constraints, Widget};
use crate::{
    alignment,
    event::{self, mouse::MouseButton},
};
use parley::FontContext;
use peniko::kurbo::{Point, Rect, RoundedRectRadii, Size};
use peniko::Color;

use super::WidgetData;

pub struct ButtonWidget<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    pub(crate) size: Size,
    pub(crate) radii: RoundedRectRadii,
    pub(crate) color: Color,
    pub(crate) on_press: Option<Message>,
    child: ContainerWidget<Message>,
    hovered: bool,
}

impl<Message> ButtonWidget<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    pub fn new(
        child: WidgetData<Message>,
        size: Size,
        radii: RoundedRectRadii,
        color: Color,
        on_press: Option<Message>,
    ) -> Self {
        let child = ContainerWidget::new(
            child,
            alignment::Horizontal::Center,
            alignment::Vertical::Center,
            0.0.into(),
        );
        Self {
            size,
            radii,
            color,
            on_press,
            child,
            hovered: false,
        }
    }
}

impl<Message> Widget<Message> for ButtonWidget<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn debug_name(&self) -> &str {
        "button"
    }
    fn paint(&self, paint_cx: &mut PaintCx) {
        paint_cx.fill_shape(
            &Rect::from_origin_size(Point::new(0., 0.), self.size).to_rounded_rect(self.radii),
            self.color,
        );
        if self.hovered {
            paint_cx.fill_shape(
                &Rect::from_origin_size(Point::new(0., 0.), self.size).to_rounded_rect(self.radii),
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
                min_size: self.size,
                max_size: self.size,
            },
            font_cx,
        );
        self.size
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
                if let Some(messsage) = &self.on_press {
                    event_cx.push_user_message(messsage.clone());
                }
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
