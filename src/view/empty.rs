use crate::view::View;
use crate::widget::{EmptyWidget, WidgetData};
use crate::AsAny;
use peniko::kurbo::Size;

// FIXME: This is obviously stupid. Turn this into a spacer?
impl<Message> View<Message> for Size
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn build_widget(&self) -> WidgetData<Message> {
        WidgetData::new(Box::new(EmptyWidget::new(*self)))
    }

    fn change_widget(&self, widget_data: &mut WidgetData<Message>) {
        widget_data
            .widget
            .as_any_mut()
            .downcast_mut::<EmptyWidget<Message>>()
            .unwrap()
            .set_size(*self);
        widget_data.change_flags.layout = true;
    }

    fn reconciliate(&self, old: &Box<dyn View<Message>>, widget: &mut WidgetData<Message>) {
        if self.as_any().downcast_ref::<Size>().unwrap()
            != (**old).as_any().downcast_ref::<Size>().unwrap()
        {
            self.change_widget(widget)
        }
    }
}
