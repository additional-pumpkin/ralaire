use itertools::Itertools;
use ralaire_core::AsAny;

use crate::view::View;
use crate::widget::{FlexAxis, FlexBoxWidget, FlexDirection, WidgetData};

use super::ViewSequence;
pub struct FlexBoxView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    children: ViewSequence<Message>,
    flex_direction: FlexDirection,
    // FIXME: How do flexboxes actually do spacing?
    spacing: f64,
}
impl<Message> FlexBoxView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    pub(crate) fn new(children: ViewSequence<Message>) -> FlexBoxView<Message> {
        FlexBoxView {
            children,
            flex_direction: FlexDirection::Column,
            spacing: 0.,
        }
    }
    pub fn direction(mut self, flex_direction: FlexDirection) -> Self {
        self.flex_direction = flex_direction;
        self
    }
    pub fn spacing(mut self, spacing: f64) -> Self {
        self.spacing = spacing;
        self
    }
}
impl<Message> View<Message> for FlexBoxView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn build_widget(&self) -> WidgetData<Message> {
        let children: Vec<_> = self
            .children
            .0
            .iter()
            .map(|child| child.build_widget())
            .collect();
        WidgetData::new(Box::new(FlexBoxWidget::new(
            children,
            self.flex_direction,
            self.spacing,
        )))
    }

    fn change_widget(&self, widget: &mut WidgetData<Message>) {
        let flex_box = widget
            .as_any_mut()
            .downcast_mut::<FlexBoxWidget<Message>>()
            .unwrap();
        let (flex_axis, direction_flipped) = match self.flex_direction {
            FlexDirection::Column => (FlexAxis::Vertical, false),
            FlexDirection::Row => (FlexAxis::Horizontal, false),
            FlexDirection::ColumnReversed => (FlexAxis::Vertical, true),
            FlexDirection::RowReversed => (FlexAxis::Horizontal, true),
        };
        flex_box.flex_axis = flex_axis;
        flex_box.direction_flipped = direction_flipped;
        flex_box.spacing = self.spacing;
        widget.change_flags.layout = true;
    }

    fn reconciliate(&self, old: &Box<dyn View<Message>>, widget_data: &mut WidgetData<Message>) {
        let old = (**old)
            .as_any()
            .downcast_ref::<FlexBoxView<Message>>()
            .unwrap();
        if self.spacing != old.spacing || self.flex_direction != old.flex_direction {
            self.change_widget(widget_data)
        }
        let child_widgets = (*widget_data.widget)
            .as_any_mut()
            .downcast_mut::<FlexBoxWidget<Message>>()
            .unwrap()
            .mut_children();
        let child_pairs = self
            .children
            .0
            .iter()
            .zip_longest(old.children.0.iter().zip(0..child_widgets.len()));
        for child_pair in child_pairs {
            match child_pair {
                itertools::EitherOrBoth::Both(new, (old, idx)) => {
                    if new.as_any().type_id() == old.as_any().type_id() {
                        new.reconciliate(old, &mut child_widgets[idx]);
                    } else {
                        let new_widget = new.build_widget();
                        child_widgets[idx] = new_widget;
                    }
                }
                itertools::EitherOrBoth::Left(new_child) => {
                    let new_widget = new_child.build_widget();
                    child_widgets.push(new_widget);
                }
                itertools::EitherOrBoth::Right((_old_child, _old_widget)) => {
                    child_widgets.pop();
                }
            }
        }
    }
}
