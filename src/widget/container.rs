use crate::widget::Widget;
use crate::{alignment, event, Padding};

use parley::FontContext;
use vello::kurbo::{Point, Size};

use super::WidgetData;

pub struct Container<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    size: Size,
    pub(crate) h_alignment: alignment::Horizontal,
    pub(crate) v_alignment: alignment::Vertical,
    pub(crate) padding: Padding,
    child: WidgetData<Message>,
}

impl<Message> Container<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    pub fn new(
        child: WidgetData<Message>,
        h_alignment: alignment::Horizontal,
        v_alignment: alignment::Vertical,
        padding: Padding,
    ) -> Self {
        Container {
            size: Size::ZERO,
            h_alignment,
            v_alignment,
            padding,
            child,
        }
    }
}

impl<Message> Widget<Message> for Container<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn paint(&mut self, scene: &mut vello::Scene) {
        self.child.paint(scene);
    }
    fn debug_name(&self) -> &str {
        "container"
    }
    fn children(&self) -> Vec<&WidgetData<Message>> {
        vec![&self.child]
    }
    fn children_mut(&mut self) -> Vec<&mut WidgetData<Message>> {
        vec![&mut self.child]
    }
    fn layout(&mut self, size_hint: Size, font_cx: &mut FontContext) -> Size {
        self.size = size_hint;
        self.child.size = self.child.layout(
            Size::new(
                size_hint.width - self.padding.horizontal(),
                size_hint.height - self.padding.vertical(),
            ),
            font_cx,
        );

        let padding = self.padding;
        let x = match self.h_alignment {
            alignment::Horizontal::Left => padding.left,
            alignment::Horizontal::Center => {
                (self.size.width - padding.horizontal() - self.child.size.width) / 2. + padding.left
            }
            alignment::Horizontal::Right => self.size.width - padding.right - self.child.size.width,
        };
        let y = match self.v_alignment {
            alignment::Vertical::Top => padding.top,
            alignment::Vertical::Center => {
                (self.size.height - padding.vertical() - self.child.size.height) / 2. + padding.top
            }
            alignment::Vertical::Bottom => {
                self.size.height - padding.bottom - self.child.size.height
            }
        };
        self.child.position = Point::new(x.max(0.), y.max(0.));
        self.size
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
