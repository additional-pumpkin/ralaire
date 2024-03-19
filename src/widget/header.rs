use crate::widget::{Constraints, Length, Widget, WidgetSize};
use parley::FontContext;
use ralaire_core::{Point, RenderCx, Size};

use super::{window_controls::WindowControlsWidget, WidgetData};

const HEADER_HEIGHT: f64 = 32.;

#[derive(Debug)]
/// like bar but includes window controls (for example minimise, maximise, close)
pub struct HeaderWidget<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    left: Option<WidgetData<Message>>,
    middle: Option<WidgetData<Message>>,
    right: Option<WidgetData<Message>>,
    _window_controls: WindowControlsWidget<Message>,
}

#[allow(dead_code)]
impl<Message> HeaderWidget<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    pub fn new(
        left: Option<WidgetData<Message>>,
        middle: Option<WidgetData<Message>>,
        right: Option<WidgetData<Message>>,
    ) -> Self {
        Self {
            left,
            middle,
            right,
            _window_controls: WindowControlsWidget::new(),
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

impl<Message> Widget<Message> for HeaderWidget<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn draw(&self, _render_cx: &mut RenderCx) {}

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

    fn size_hint(&self) -> WidgetSize {
        WidgetSize {
            width: Length::Flexible(1),
            height: Length::Flexible(1),
        }
    }

    fn layout(&mut self, constraints: Constraints, font_cx: &mut FontContext) {
        if !constraints.max_size.is_finite() {
            tracing::error!("Bar widget: max size is infinite");
        }
        let side_constraints = Constraints {
            min_size: Size::ZERO,
            max_size: Size {
                width: f64::INFINITY,
                height: HEADER_HEIGHT,
            },
        };
        let left_width;
        let right_width;
        if let Some(left) = &mut self.left {
            left.widget.layout(side_constraints, font_cx);
            if let Length::Fixed(width) = left.widget.size_hint().width {
                left_width = width;
            } else {
                tracing::error!("Bar widget: child has flexible width");
                left_width = 0.;
            }
        } else {
            left_width = 0.;
        }
        if let Some(right) = &mut self.right {
            right.widget.layout(side_constraints, font_cx);
            if let Length::Fixed(width) = right.widget.size_hint().width {
                right_width = width;
            } else {
                tracing::error!("Bar widget: child has flexible width");
                right_width = 0.;
            }
        } else {
            right_width = 0.;
        }
        let max_width = f64::max(left_width, right_width + 200.);
        let middle_width = constraints.max_size.width - max_width * 2.;
        if let Some(left) = &mut self.left {
            left.size = Size::new(max_width, HEADER_HEIGHT);
            left.position = Point::new(0., 0.);
        }
        if let Some(right) = &mut self.right {
            right.size = Size::new(max_width, HEADER_HEIGHT);
            right.position = Point::new(max_width + middle_width, 0.);
        }
        let middle_constraints = Constraints {
            min_size: Size::ZERO,
            max_size: Size {
                width: middle_width,
                height: HEADER_HEIGHT,
            },
        };
        // TODO: handle all sizes withing constraints
        if let Some(middle) = &mut self.middle {
            middle.widget.layout(middle_constraints, font_cx);
            middle.size = Size::new(middle_width, HEADER_HEIGHT);
            middle.position = Point::new(max_width, 0.);
        }
    }
}
