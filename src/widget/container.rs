use std::marker::PhantomData;

// use ralaire_core::{Affine, Color};
use crate::widget::{Constraints, Length, Widget, WidgetCx, WidgetSize};
use parley::FontContext;
use ralaire_core::{alignment, Padding, Point, Size, WidgetId};

#[derive(Debug)]
pub struct Container<Message> {
    size: Size,
    v_alignment: alignment::Vertical,
    h_alignment: alignment::Horizontal,
    padding: Padding,
    child: WidgetId,
    phantom_data: PhantomData<Message>,
}

impl<Message> Container<Message> {
    pub fn new(child: WidgetId) -> Self {
        Container {
            size: Size::ZERO,
            v_alignment: alignment::Vertical::Center,
            h_alignment: alignment::Horizontal::Center,
            padding: Padding::ZERO,
            child,
            phantom_data: PhantomData,
        }
    }
    pub fn pad<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn v_align(mut self, v_alignment: alignment::Vertical) -> Self {
        self.v_alignment = v_alignment;
        self
    }
    pub fn h_align(mut self, h_alignment: alignment::Horizontal) -> Self {
        self.h_alignment = h_alignment;
        self
    }
}

impl<Message> Widget<Message> for Container<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn size_hint(&self) -> WidgetSize {
        WidgetSize {
            width: Length::Fixed(self.size.width),
            height: Length::Fixed(self.size.height),
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        vec![self.child]
    }
    fn layout(
        &mut self,
        widget_cx: &mut WidgetCx<Message>,
        constraints: Constraints,
        font_cx: &mut FontContext,
    ) {
        let size = constraints.max_size;
        self.size = size;
        let child_max_size = size - Size::new(self.padding.horizontal(), self.padding.vertical());
        widget_cx.layout(
            self.child,
            Constraints {
                min_size: Size::ZERO,
                max_size: size,
            },
            font_cx,
        );

        let WidgetSize { width, height } = widget_cx.size_hint(self.child);
        *widget_cx.size_mut(self.child) = match (width, height) {
            (Length::Fixed(w), Length::Fixed(h)) => Size::new(w, h),
            (Length::Fixed(w), Length::Flexible(_)) => Size::new(w, child_max_size.height),
            (Length::Flexible(_), Length::Fixed(h)) => Size::new(child_max_size.width, h),
            (Length::Flexible(_), Length::Flexible(_)) => child_max_size,
        };
        let child_size = widget_cx.size(self.child);
        widget_cx.layout(
            self.child,
            Constraints {
                min_size: child_size,
                max_size: child_size,
            },
            font_cx,
        );
        let padding = self.padding.fit(child_size, self.size);
        let x = match self.h_alignment {
            alignment::Horizontal::Left => padding.left,
            alignment::Horizontal::Center => {
                (self.size.width - padding.horizontal() - child_size.width) / 2. + padding.left
            }
            alignment::Horizontal::Right => self.size.width - padding.right - child_size.width,
        };
        let y = match self.v_alignment {
            alignment::Vertical::Top => padding.top,
            alignment::Vertical::Center => {
                (self.size.height - padding.vertical() - child_size.height) / 2. + padding.top
            }
            alignment::Vertical::Bottom => self.size.height - padding.bottom - child_size.height,
        };
        *widget_cx.position_mut(self.child) = Point::new(x.max(0.), y.max(0.));
    }
}
