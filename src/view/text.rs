use crate::view::View;
use crate::widget::{TextWidget, WidgetData};
impl<Message> View<Message> for String
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn build_widget(&self) -> WidgetData<Message> {
        WidgetData::new(Box::new(TextWidget::new(self.clone())))
    }

    fn change_widget(&self, widget_data: &mut WidgetData<Message>) {
        (*widget_data.widget)
            .as_any_mut()
            .downcast_mut::<TextWidget>()
            .unwrap()
            .set_text(self.clone());
        widget_data.change_flags.layout = true;
        widget_data.change_flags.draw = true;
    }

    fn reconciliate(&self, old: &Box<dyn View<Message>>, widget: &mut WidgetData<Message>) {
        if self != (**old).as_any().downcast_ref::<String>().unwrap() {
            self.change_widget(widget)
        }
    }
}
