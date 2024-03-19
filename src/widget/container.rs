use std::vec;

use crate::widget::{Constraints, Length, Widget, WidgetSize};
use parley::FontContext;
use ralaire_core::{alignment, Padding, Point, Size};

use super::WidgetData;

#[derive(Debug)]
pub struct ContainerWidget<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    size: Size,
    pub(crate) h_alignment: alignment::Horizontal,
    pub(crate) v_alignment: alignment::Vertical,
    pub(crate) padding: Padding,
    child: WidgetData<Message>,
}

impl<Message> ContainerWidget<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    pub fn new(
        child: WidgetData<Message>,
        h_alignment: alignment::Horizontal,
        v_alignment: alignment::Vertical,
        padding: Padding,
    ) -> Self {
        ContainerWidget {
            size: Size::ZERO,
            h_alignment,
            v_alignment,
            padding,
            child,
        }
    }
}

impl<Message> Widget<Message> for ContainerWidget<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn size_hint(&self) -> WidgetSize {
        WidgetSize {
            width: Length::Fixed(self.size.width),
            height: Length::Fixed(self.size.height),
        }
    }

    fn children(&self) -> Vec<&WidgetData<Message>> {
        vec![&self.child]
    }
    fn children_mut(&mut self) -> Vec<&mut WidgetData<Message>> {
        vec![&mut self.child]
    }
    fn layout(&mut self, constraints: Constraints, font_cx: &mut FontContext) {
        let size = constraints.max_size;
        self.size = size;
        let child_max_size = size - Size::new(self.padding.horizontal(), self.padding.vertical());

        self.child.widget.layout(
            Constraints {
                min_size: Size::ZERO,
                max_size: size,
            },
            font_cx,
        );

        let WidgetSize { width, height } = self.child.widget.size_hint();
        self.child.size = match (width, height) {
            (Length::Fixed(w), Length::Fixed(h)) => Size::new(w, h),
            (Length::Fixed(w), Length::Flexible(_)) => Size::new(w, child_max_size.height),
            (Length::Flexible(_), Length::Fixed(h)) => Size::new(child_max_size.width, h),
            (Length::Flexible(_), Length::Flexible(_)) => child_max_size,
        };
        self.child.widget.layout(
            Constraints {
                min_size: self.child.size,
                max_size: self.child.size,
            },
            font_cx,
        );
        let padding = self.padding.fit(self.child.size, self.size);
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
    }
}
