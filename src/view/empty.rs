use crate::view::View;
use crate::widget::{EmptyWidget, Length, WidgetData, WidgetSize};
use ralaire_core::AsAny;
impl<Message> View<Message> for Length
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn build_widget(&self) -> WidgetData<Message> {
        WidgetData::new(Box::new(EmptyWidget::new(WidgetSize {
            width: self.clone(),
            height: self.clone(),
        })))
    }

    fn change_widget(&self, widget_data: &mut WidgetData<Message>) {
        widget_data
            .widget
            .as_any_mut()
            .downcast_mut::<EmptyWidget<Message>>()
            .unwrap()
            .set_size_hint(WidgetSize {
                width: self.clone(),
                height: self.clone(),
            });
        widget_data.change_flags.layout = true;
    }

    fn reconciliate(&self, old: &Box<dyn View<Message>>, widget: &mut WidgetData<Message>) {
        if self.as_any().downcast_ref::<Length>().unwrap()
            != (**old).as_any().downcast_ref::<Length>().unwrap()
        {
            self.change_widget(widget)
        }
    }
}
