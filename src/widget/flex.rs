use crate::event;
use crate::widget::{Widget, WidgetData, WidgetMarker};
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CrossAxisAlignment {
    Start,
    End,
    Center,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JustifyContent {
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

pub struct FlexChild<State> {
    pub widget: WidgetData<State>,
    pub flex_factor: Option<f64>,
    pub cross_axis_alignment: Option<CrossAxisAlignment>,
}
pub struct Flex<State> {
    pub(crate) main_axis: FlexAxis,
    pub(crate) direction_flipped: bool,
    pub(crate) cross_axis_alignment: CrossAxisAlignment,
    pub(crate) justify_content: JustifyContent,
    children: Vec<FlexChild<State>>,
}

impl<State> Flex<State> {
    pub fn new(
        children: Vec<FlexChild<State>>,
        flex_direction: FlexDirection,
        cross_axis_alignment: CrossAxisAlignment,
        justify_content: JustifyContent,
    ) -> Self {
        let (main_axis, direction_flipped) = match flex_direction {
            FlexDirection::Column => (FlexAxis::Vertical, false),
            FlexDirection::Row => (FlexAxis::Horizontal, false),
            FlexDirection::ColumnReversed => (FlexAxis::Vertical, true),
            FlexDirection::RowReversed => (FlexAxis::Horizontal, true),
        };
        Self {
            main_axis,
            direction_flipped,
            cross_axis_alignment,
            justify_content,
            children,
        }
    }
    pub fn insert_child(&mut self, idx: usize, child: FlexChild<State>) {
        self.children.insert(idx, child);
    }
    pub fn remove_child(&mut self, idx: usize) {
        self.children.remove(idx);
    }
    pub fn mutate_child(&mut self, idx: usize) -> &mut FlexChild<State> {
        self.children.get_mut(idx).unwrap()
    }
    pub fn set_flex_direction(&mut self, _flex_direction: FlexDirection) {}
}

impl<State> WidgetMarker for Flex<State> {}
impl<State: 'static> Widget<State> for Flex<State> {
    fn debug_name(&self) -> &str {
        "flexbox"
    }
    fn paint(&mut self, scene: &mut vello::Scene) {
        for child in self.children_mut() {
            child.paint(scene);
        }
    }

    fn children(&self) -> Vec<&WidgetData<State>> {
        self.children.iter().map(|child| &child.widget).collect()
    }

    fn children_mut(&mut self) -> Vec<&mut WidgetData<State>> {
        self.children
            .iter_mut()
            .map(|child| &mut child.widget)
            .collect()
    }

    fn layout(&mut self, suggested_size: Size, font_context: &mut FontContext) -> Size {
        if !suggested_size.is_finite() {
            panic!("FIXME: size is infinite");
        }
        let (main_axis_size, cross_axis_size) = match self.main_axis {
            FlexAxis::Horizontal => (suggested_size.width, suggested_size.height),
            FlexAxis::Vertical => (suggested_size.height, suggested_size.width),
        };
        let mut main_axis_off = 0.;
        let mut sizes = vec![];
        let mut total_fixed_main_axis = 0.;
        let mut total_flex_main_axis = 0.;
        let mut no_flex_children = true;
        for child in self.children.iter_mut() {
            let child_size = child.widget.layout(suggested_size, font_context);
            sizes.push(child_size);
            match self.main_axis {
                FlexAxis::Horizontal => match child.flex_factor {
                    Some(flex_factor) => {
                        no_flex_children = false;
                        total_flex_main_axis += flex_factor;
                    }
                    None => {
                        total_fixed_main_axis += child_size.width;
                    }
                },
                FlexAxis::Vertical => match child.flex_factor {
                    Some(flex_factor) => {
                        no_flex_children = false;
                        total_flex_main_axis += flex_factor;
                    }
                    None => {
                        total_fixed_main_axis += child_size.height;
                    }
                },
            }
        }
        let flexible_space = main_axis_size - total_fixed_main_axis;
        let flex_unit_main_axis = flexible_space / total_flex_main_axis as f64;
        let justify_space = if no_flex_children {
            let free_space = main_axis_size - total_fixed_main_axis;
            match self.justify_content {
                JustifyContent::Start => 0.,
                JustifyContent::End => {
                    main_axis_off = free_space;
                    0.
                }
                JustifyContent::Center => {
                    main_axis_off = (free_space) / 2.;
                    0.
                }
                JustifyContent::SpaceBetween => free_space / (self.children.len() as f64 - 1.0),
                JustifyContent::SpaceAround => {
                    main_axis_off = (free_space / (self.children.len() as f64)) / 2.;
                    free_space / (self.children.len() as f64)
                }
                JustifyContent::SpaceEvenly => {
                    main_axis_off = free_space / (self.children.len() as f64 + 1.);
                    main_axis_off
                }
            }
        } else {
            0.
        };
        for (child, size) in self.children.iter_mut().zip(sizes.clone()) {
            let child_size;
            match self.main_axis {
                FlexAxis::Horizontal => {
                    let cross_axis_alignment = if let Some(a) = child.cross_axis_alignment {
                        a
                    } else {
                        self.cross_axis_alignment
                    };
                    let cross_axis_offset = match cross_axis_alignment {
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
                    let cross_axis_alignment = if let Some(a) = child.cross_axis_alignment {
                        a
                    } else {
                        self.cross_axis_alignment
                    };
                    let cross_axis_offset = match cross_axis_alignment {
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
            child.widget.layout(child_size, font_context);
            match self.main_axis {
                FlexAxis::Horizontal => {
                    main_axis_off += child_size.width + justify_space;
                }
                FlexAxis::Vertical => {
                    main_axis_off += child_size.height + justify_space;
                }
            }
        }

        let flex_width;
        let flex_height;
        match self.main_axis {
            FlexAxis::Vertical => {
                if no_flex_children {
                    flex_height = sizes.iter().map(|size| size.height).sum::<f64>();
                    flex_width = sizes
                        .iter()
                        .map(|size| size.width)
                        .reduce(f64::max)
                        .unwrap_or(0.);
                } else {
                    flex_height = suggested_size.height;
                    flex_width = sizes
                        .iter()
                        .map(|size| size.width)
                        .reduce(f64::max)
                        .unwrap_or(0.);
                }
            }
            FlexAxis::Horizontal => {
                if no_flex_children {
                    flex_width = sizes.iter().map(|size| size.width).sum::<f64>();
                    flex_height = sizes
                        .iter()
                        .map(|size| size.height)
                        .reduce(f64::max)
                        .unwrap_or(0.);
                } else {
                    flex_width = suggested_size.width;
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
        _event_context: &mut event::EventContext,
        _event: event::WidgetEvent,
        _state: &mut State,
    ) -> event::Status {
        event::Status::Ignored
    }

    fn set_hover(&mut self, _hover: bool) -> event::Status {
        event::Status::Ignored
    }
}

// impl<State: 'static> Widget<State> for FlexChild<State> {
//     fn layout(&mut self, suggested_size: Size, font_context: &mut FontContext) -> Size {
//         self.widget.inner.layout(suggested_size, font_context)
//     }

//     fn event(
//         &mut self,
//         event_context: &mut event::EventContext,
//         event: event::WidgetEvent,
//         state: &mut State,
//     ) -> event::Status {
//         self.widget.inner.event(event_context, event, state)
//     }

//     fn set_hover(&mut self, hover: bool) -> event::Status {
//         self.widget.inner.set_hover(hover)
//     }

//     fn paint(&mut self, scene: &mut vello::Scene) {
//         self.widget.inner.paint(scene);
//     }

//     fn children(&self) -> Vec<&WidgetData<State>> {
//         self.widget.inner.children()
//     }

//     fn children_mut(&mut self) -> Vec<&mut WidgetData<State>> {
//         self.widget.inner.children_mut()
//     }

//     fn debug_name(&self) -> &str {
//         self.widget.inner.debug_name()
//     }
// }
