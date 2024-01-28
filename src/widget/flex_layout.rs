use crate::widget::Widget;
use parley::FontContext;
use ralaire_core::{Point, RenderCx, Size};

use super::widget::{Constraints, Length, WidgetData, WidgetSize};

#[allow(dead_code)]
#[derive(Debug)]
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
    children: Vec<WidgetData<Message>>,
    flex_axis: FlexAxis,
    direction_flipped: bool,
}

impl<Message> FlexBox<Message>
where
    Message: Clone,
{
    pub fn new(children: Vec<WidgetData<Message>>, flex_direction: FlexDirection) -> Self {
        let (flex_axis, direction_flipped) = match flex_direction {
            FlexDirection::Column => (FlexAxis::Vertical, false),
            FlexDirection::Row => (FlexAxis::Horizontal, false),
            FlexDirection::ColumnReversed => (FlexAxis::Vertical, true),
            FlexDirection::RowReversed => (FlexAxis::Horizontal, true),
        };
        FlexBox {
            spacing: 0.,
            children,
            flex_axis,
            direction_flipped,
        }
    }
    pub fn with_spacing(mut self, spacing: f64) -> Self {
        self.spacing = spacing;
        self
    }
}

impl<Message> Widget<Message> for FlexBox<Message>
where
    Message: std::fmt::Debug + Clone,
{
    fn draw(&self, _render_cx: &mut RenderCx) {}

    fn children(&self) -> Vec<&WidgetData<Message>> {
        self.children.iter().collect()
    }

    fn children_mut(&mut self) -> Vec<&mut WidgetData<Message>> {
        self.children.iter_mut().collect()
    }

    fn size_hint(&self) -> WidgetSize {
        WidgetSize {
            width: Length::Flexible(1),
            height: Length::Flexible(1),
        }
    }

    fn layout(&mut self, constraints: Constraints, font_cx: &mut FontContext) {
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
            for child in self.children.iter_mut() {
                let max_size = if let FlexAxis::Horizontal = self.flex_axis {
                    Size::new(size.height, f64::INFINITY)
                } else {
                    Size::new(f64::INFINITY, size.width)
                };
                child.widget.layout(
                    Constraints {
                        min_size: Size::ZERO,
                        max_size,
                    },
                    font_cx,
                );
                let size_hint = child.widget.size_hint();
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
            for (child, size_hint) in self.children.iter_mut().zip(sizes) {
                let child_size;
                if let FlexAxis::Horizontal = self.flex_axis {
                    if self.direction_flipped {
                        child.position = Point::new(main_axis_size - main_axis_off, 0.);
                    } else {
                        child.position = Point::new(main_axis_off, 0.);
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
                        child.position = Point::new(0., main_axis_size - main_axis_off);
                    } else {
                        child.position = Point::new(0., main_axis_off);
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
                child.size = child_size;
                child.widget.layout(child_constraints, font_cx);
                if let FlexAxis::Horizontal = self.flex_axis {
                    main_axis_off += child_size.width + self.spacing;
                } else {
                    main_axis_off += child_size.height + self.spacing;
                }
            }
        }
    }
}
