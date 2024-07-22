use super::WidgetData;
use crate::event;
use crate::widget::Widget;
use parley::FontContext;
use vello::kurbo::{Point, Size};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexDirection {
    Column,
    Row,
    ColumnReversed,
    RowReversed,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexAxis {
    Vertical,
    Horizontal,
}

pub enum CrossAxisAlignment {
    Start,
    Center,
    End,
}

pub struct FlexChild<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    pub widget: WidgetData<Message>,
    pub flex_factor: Option<f64>,
}
pub struct Flex<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    pub(crate) spacing: f64,
    pub(crate) main_axis: FlexAxis,
    pub(crate) direction_flipped: bool,
    pub(crate) cross_axis_alignment: CrossAxisAlignment,
    children: Vec<FlexChild<Message>>,
}

impl<Message> Flex<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    pub fn new(
        children: Vec<FlexChild<Message>>,
        flex_direction: FlexDirection,
        spacing: f64,
    ) -> Self {
        let (main_axis, direction_flipped) = match flex_direction {
            FlexDirection::Column => (FlexAxis::Vertical, false),
            FlexDirection::Row => (FlexAxis::Horizontal, false),
            FlexDirection::ColumnReversed => (FlexAxis::Vertical, true),
            FlexDirection::RowReversed => (FlexAxis::Horizontal, true),
        };
        Self {
            spacing,
            children,
            main_axis,
            cross_axis_alignment: CrossAxisAlignment::Center,
            direction_flipped,
        }
    }
    pub fn set_flex_direction(&mut self, _flex_direction: FlexDirection) {}
    pub fn mut_children(&mut self) -> &mut Vec<FlexChild<Message>> {
        &mut self.children
    }
}

impl<Message> Widget<Message> for Flex<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn debug_name(&self) -> &str {
        "flexbox"
    }
    fn paint(&mut self, scene: &mut vello::Scene) {
        for child in self.children_mut() {
            child.paint(scene);
        }
    }

    fn children(&self) -> Vec<&WidgetData<Message>> {
        self.children.iter().map(|child| &child.widget).collect()
    }

    fn children_mut(&mut self) -> Vec<&mut WidgetData<Message>> {
        self.children
            .iter_mut()
            .map(|child| &mut child.widget)
            .collect()
    }

    fn layout(&mut self, size_hint: Size, font_cx: &mut FontContext) -> Size {
        let (main_axis_size, cross_axis_size) = match self.main_axis {
            FlexAxis::Horizontal => (size_hint.width, size_hint.height),
            FlexAxis::Vertical => (size_hint.height, size_hint.width),
        };
        let total_spacing = (self.children.len() + 1) as f64 * self.spacing;
        let mut main_axis_off = self.spacing;
        let mut sizes = vec![];
        let mut total_fixed_main_axis = 0.;
        let mut total_flex_main_axis = 0.;
        for child in self.children.iter_mut() {
            let child_size = child.widget.layout(size_hint, font_cx);
            sizes.push(child_size);
            match self.main_axis {
                FlexAxis::Horizontal => match child.flex_factor {
                    Some(flex_factor) => {
                        total_flex_main_axis += flex_factor;
                    }
                    None => {
                        total_fixed_main_axis += child_size.width;
                    }
                },
                FlexAxis::Vertical => match child.flex_factor {
                    Some(flex_factor) => {
                        total_flex_main_axis += flex_factor;
                    }
                    None => {
                        total_fixed_main_axis += child_size.height;
                    }
                },
            }
        }
        let flexible_space = main_axis_size - total_fixed_main_axis - total_spacing;
        let flex_unit_main_axis = flexible_space / total_flex_main_axis as f64;
        for (child, size) in self.children.iter_mut().zip(sizes.clone()) {
            let child_size;
            match self.main_axis {
                FlexAxis::Horizontal => {
                    let cross_axis_offset = match self.cross_axis_alignment {
                        CrossAxisAlignment::Start => 0.0,
                        CrossAxisAlignment::Center => (cross_axis_size - size.height) / 2.0,
                        CrossAxisAlignment::End => cross_axis_size - size.height,
                    };
                    if self.direction_flipped {
                        child.widget.position = Point::new(
                            main_axis_size - main_axis_off - size.width,
                            cross_axis_offset,
                        );
                    } else {
                        child.widget.position = Point::new(main_axis_off, cross_axis_offset);
                    }

                    let child_width = match child.flex_factor {
                        Some(flex_factor) => flex_unit_main_axis * flex_factor,
                        None => size.width,
                    };
                    child_size = Size::new(child_width, size.height);
                }
                FlexAxis::Vertical => {
                    let cross_axis_offset = match self.cross_axis_alignment {
                        CrossAxisAlignment::Start => 0.0,
                        CrossAxisAlignment::Center => (cross_axis_size - size.width) / 2.0,
                        CrossAxisAlignment::End => cross_axis_size - size.width,
                    };
                    if self.direction_flipped {
                        child.widget.position = Point::new(
                            cross_axis_offset,
                            main_axis_size - main_axis_off - size.height,
                        );
                    } else {
                        child.widget.position = Point::new(cross_axis_offset, main_axis_off);
                    }

                    let child_height = match child.flex_factor {
                        Some(flex_factor) => flex_unit_main_axis * flex_factor,
                        None => size.height,
                    };

                    child_size = Size::new(size.width, child_height);
                }
            }
            child.widget.size = child_size;
            child.widget.layout(child_size, font_cx);
            match self.main_axis {
                FlexAxis::Horizontal => {
                    main_axis_off += child_size.width + self.spacing;
                }
                FlexAxis::Vertical => {
                    main_axis_off += child_size.height + self.spacing;
                }
            }
        }

        let flex_width;
        let flex_height;
        match self.main_axis {
            FlexAxis::Vertical => {
                if self
                    .children
                    .iter()
                    .all(|child| child.flex_factor.is_none())
                {
                    flex_height = sizes.iter().map(|size| size.height).sum::<f64>() + total_spacing;
                    flex_width = sizes
                        .iter()
                        .map(|size| size.width)
                        .reduce(f64::max)
                        .unwrap_or(0.);
                } else {
                    flex_height = size_hint.height;
                    flex_width = sizes
                        .iter()
                        .map(|size| size.width)
                        .reduce(f64::max)
                        .unwrap_or(0.);
                }
            }
            FlexAxis::Horizontal => {
                if self
                    .children
                    .iter()
                    .all(|child| child.flex_factor.is_none())
                {
                    flex_width = sizes.iter().map(|size| size.width).sum::<f64>() + total_spacing;
                    flex_height = sizes
                        .iter()
                        .map(|size| size.height)
                        .reduce(f64::max)
                        .unwrap_or(0.);
                } else {
                    flex_width = size_hint.width;
                    flex_height = sizes
                        .iter()
                        .map(|size| size.height)
                        .reduce(f64::max)
                        .unwrap_or(0.);
                }
            }
        }
        Size::new(flex_width, flex_height)
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
