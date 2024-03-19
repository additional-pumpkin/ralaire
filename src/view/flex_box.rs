use itertools::Itertools;
use ralaire_core::{AsAny, WidgetId};

use crate::view::View;
use crate::widget::{FlexBox, FlexDirection, WidgetCx, WidgetData};
pub struct FlexBoxView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    pub children: Vec<Box<dyn View<Message>>>,
    pub flex_direction: FlexDirection,
}
impl<Message> View<Message> for FlexBoxView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn build_widget(&self, widget_cx: &mut WidgetCx<Message>) -> WidgetData<Message> {
        let children_data: Vec<_> = self
            .children
            .iter()
            .map(|child| child.build_widget(widget_cx))
            .collect();
        let children_ids = children_data.iter().map(|data| data.id).collect();
        for child in children_data.into_iter() {
            widget_cx.add_widget(child)
        }
        WidgetData::new(Box::new(FlexBox::new(children_ids, self.flex_direction)))
    }

    fn change_widget(&self, widget: &mut WidgetData<Message>) {
        widget
            .as_any_mut()
            .downcast_mut::<FlexBox<Message>>()
            .unwrap()
            .set_flex_direction(self.flex_direction);
        widget.change_flags.layout = true;
    }

    fn reconciliate(
        &self,
        old: &Box<dyn View<Message>>,
        widget_id: WidgetId,
        widget_cx: &mut WidgetCx<Message>,
    ) {
        if self
            .as_any()
            .downcast_ref::<FlexBoxView<Message>>()
            .unwrap()
            .flex_direction
            != (**old)
                .as_any()
                .downcast_ref::<FlexBoxView<Message>>()
                .unwrap()
                .flex_direction
        {
            self.change_widget(widget_cx.widget(widget_id))
        }
        let child_widgets = widget_cx.children(widget_id);
        let child_ids: Vec<_> = child_widgets.iter().map(|data| data.id).collect();
        let mut new_child_ids = vec![];
        let child_pairs = self
            .as_any()
            .downcast_ref::<FlexBoxView<Message>>()
            .unwrap()
            .children
            .iter()
            .zip_longest(
                (**old)
                    .as_any()
                    .downcast_ref::<FlexBoxView<Message>>()
                    .unwrap()
                    .children
                    .iter()
                    .zip(child_ids.into_iter()),
            );
        for child_pair in child_pairs {
            match child_pair {
                itertools::EitherOrBoth::Both(new_child, (old_child, old_id)) => {
                    if new_child.as_any().type_id() == old_child.as_any().type_id() {
                        new_child_ids.push(old_id);
                        new_child.reconciliate(old_child, old_id, widget_cx);
                    } else {
                        let new_widget = new_child.build_widget(widget_cx);
                        new_child_ids.push(new_widget.id);
                        widget_cx.remove_widget(old_id);
                        widget_cx.add_widget(new_widget);
                    }
                }
                itertools::EitherOrBoth::Left(new_child) => {
                    let new_widget = new_child.build_widget(widget_cx);
                    new_child_ids.push(new_widget.id);
                    widget_cx.add_widget(new_widget);
                }
                itertools::EitherOrBoth::Right((_old_child, old_id)) => {
                    widget_cx.remove_widget(old_id);
                }
            }
        }
        let widget = &mut widget_cx.widget(widget_id).widget;
        widget.set_children(new_child_ids);
    }
}
