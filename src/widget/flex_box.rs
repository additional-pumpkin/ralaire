use std::marker::PhantomData;

use crate::widget::{Constraints, Length, Widget, WidgetCx, WidgetSize};
use parley::FontContext;
use ralaire_core::{Point, RenderCx, Size, WidgetId};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexDirection {
    Column,
    Row,
    ColumnReversed,
    RowReversed,
}
#[derive(Debug)]
pub enum FlexAxis {
    Vertical,
    Horizontal,
}

#[derive(Debug)]
pub struct FlexBox<Message> {
    spacing: f64,
    children: Vec<WidgetId>,
    flex_axis: FlexAxis,
    direction_flipped: bool,
    phantom_data: PhantomData<Message>,
}

impl<Message> FlexBox<Message> {
    pub fn new(children: Vec<WidgetId>, flex_direction: FlexDirection) -> Self {
        let (flex_axis, direction_flipped) = match flex_direction {
            FlexDirection::Column => (FlexAxis::Vertical, false),
            FlexDirection::Row => (FlexAxis::Horizontal, false),
            FlexDirection::ColumnReversed => (FlexAxis::Vertical, true),
            FlexDirection::RowReversed => (FlexAxis::Horizontal, true),
        };
        Self {
            spacing: 0.,
            children,
            flex_axis,
            direction_flipped,
            phantom_data: PhantomData,
        }
    }
    pub fn with_spacing(mut self, spacing: f64) -> Self {
        self.spacing = spacing;
        self
    }
    pub fn set_flex_direction(&mut self, flex_direction: FlexDirection) {
        let (flex_axis, direction_flipped) = match flex_direction {
            FlexDirection::Column => (FlexAxis::Vertical, false),
            FlexDirection::Row => (FlexAxis::Horizontal, false),
            FlexDirection::ColumnReversed => (FlexAxis::Vertical, true),
            FlexDirection::RowReversed => (FlexAxis::Horizontal, true),
        };
        self.flex_axis = flex_axis;
        self.direction_flipped = direction_flipped;
    }
}

impl<Message> Widget<Message> for FlexBox<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn draw(&self, _render_cx: &mut RenderCx) {}

    fn children(&self) -> Vec<WidgetId> {
        self.children.iter().map(|id| id.clone()).collect()
    }

    fn size_hint(&self) -> WidgetSize {
        WidgetSize {
            width: Length::Flexible(1),
            height: Length::Flexible(1),
        }
    }

    fn layout(
        &mut self,
        widget_cx: &mut WidgetCx<Message>,
        constraints: Constraints,
        font_cx: &mut FontContext,
    ) {
        if constraints.max_size.is_finite() {
            let size = constraints.max_size;
            let (main_axis_size, cross_axis_size) = if let FlexAxis::Horizontal = self.flex_axis {
                (size.width, size.height)
            } else {
                (size.height, size.width)
            };
            let total_spacing = (self.children.len() + 1) as f64 * self.spacing;
            let mut main_axis_off = self.spacing;
            let mut sizes = vec![];
            let mut total_fixed_main_size = 0.;
            let mut total_flex_main_axis = 0;
            let mut max_flex_cross_axis = 0;
            for child in self.children.iter() {
                let max_size = if let FlexAxis::Horizontal = self.flex_axis {
                    Size::new(size.height, f64::INFINITY)
                } else {
                    Size::new(f64::INFINITY, size.width)
                };
                widget_cx.layout(
                    *child,
                    Constraints {
                        min_size: Size::ZERO,
                        max_size,
                    },
                    font_cx,
                );
                let size_hint = widget_cx.size_hint(*child);
                sizes.push(size_hint);
                if let FlexAxis::Horizontal = self.flex_axis {
                    match size_hint.width {
                        Length::Fixed(w) => {
                            total_fixed_main_size += w;
                        }
                        Length::Flexible(flex) => total_flex_main_axis += flex,
                    }
                    if let Length::Flexible(f) = size_hint.height {
                        if f > max_flex_cross_axis {
                            max_flex_cross_axis = f;
                        }
                    }
                } else {
                    match size_hint.height {
                        Length::Fixed(h) => {
                            total_fixed_main_size += h;
                        }
                        Length::Flexible(flex) => total_flex_main_axis += flex,
                    }
                    if let Length::Flexible(f) = size_hint.width {
                        if f > max_flex_cross_axis {
                            max_flex_cross_axis = f;
                        }
                    }
                }
            }
            let flexible_space = main_axis_size - total_fixed_main_size - total_spacing;
            let flex_unit_main_axis = flexible_space / total_flex_main_axis as f64;
            let flex_unit_cross_axis = cross_axis_size / max_flex_cross_axis as f64;
            for (child, size_hint) in self.children.iter().zip(sizes) {
                let child_size;
                if let FlexAxis::Horizontal = self.flex_axis {
                    if self.direction_flipped {
                        *widget_cx.position_mut(*child) =
                            Point::new(main_axis_size - main_axis_off, 0.);
                    } else {
                        *widget_cx.position_mut(*child) = Point::new(main_axis_off, 0.);
                    }

                    let child_width = match size_hint.width {
                        Length::Fixed(w) => w,
                        Length::Flexible(f) => flex_unit_main_axis * f as f64,
                    };

                    let child_height = match size_hint.height {
                        Length::Fixed(h) => h,
                        Length::Flexible(f) => flex_unit_cross_axis * f as f64,
                    };
                    child_size = Size::new(child_width, child_height);
                } else {
                    if self.direction_flipped {
                        *widget_cx.position_mut(*child) =
                            Point::new(0., main_axis_size - main_axis_off);
                    } else {
                        *widget_cx.position_mut(*child) = Point::new(0., main_axis_off);
                    }

                    let child_width = match size_hint.width {
                        Length::Fixed(w) => w,
                        Length::Flexible(f) => flex_unit_cross_axis * f as f64,
                    };

                    let child_height = match size_hint.height {
                        Length::Fixed(h) => h,
                        Length::Flexible(f) => flex_unit_main_axis * f as f64,
                    };
                    child_size = Size::new(child_width, child_height);
                }
                let child_constraints = Constraints {
                    min_size: child_size,
                    max_size: child_size,
                };
                *widget_cx.size_mut(*child) = child_size;
                widget_cx.layout(*child, child_constraints, font_cx);
                if let FlexAxis::Horizontal = self.flex_axis {
                    main_axis_off += child_size.width + self.spacing;
                } else {
                    main_axis_off += child_size.height + self.spacing;
                }
            }
        }
    }
}
