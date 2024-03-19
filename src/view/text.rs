use crate::view::View;
use crate::widget::{Text, WidgetCx, WidgetData};
use ralaire_core::{AsAny, WidgetId};
impl<Message> View<Message> for String
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn build_widget(&self, _widget_cx: &mut WidgetCx<Message>) -> WidgetData<Message> {
        WidgetData::new(Box::new(Text::new(self.clone())))
    }

    fn change_widget(&self, widget_data: &mut WidgetData<Message>) {
        widget_data
            .widget
            .as_any_mut()
            .downcast_mut::<Text>()
            .unwrap()
            .set_text(self.clone());
        widget_data.change_flags.layout = true;
        widget_data.change_flags.draw = true;
    }

    fn reconciliate(
        &self,
        old: &Box<dyn View<Message>>,
        widget_id: WidgetId,
        widget_cx: &mut WidgetCx<Message>,
    ) {
        if self.as_any().downcast_ref::<String>().unwrap()
            != (**old).as_any().downcast_ref::<String>().unwrap()
        {
            self.change_widget(widget_cx.widget(widget_id))
        }
    }
}
