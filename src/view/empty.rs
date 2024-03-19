use crate::view::View;
use crate::widget::{Empty, Length, WidgetCx, WidgetData, WidgetSize};
use ralaire_core::{AsAny, WidgetId};
impl<Message> View<Message> for Length
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn build_widget(&self, _widget_cx: &mut WidgetCx<Message>) -> WidgetData<Message> {
        WidgetData::new(Box::new(Empty::new(WidgetSize {
            width: self.clone(),
            height: self.clone(),
        })))
    }

    fn change_widget(&self, widget_data: &mut WidgetData<Message>) {
        widget_data
            .widget
            .as_any_mut()
            .downcast_mut::<Empty<Message>>()
            .unwrap()
            .set_size_hint(WidgetSize {
                width: self.clone(),
                height: self.clone(),
            });
        widget_data.change_flags.layout = true;
    }

    fn reconciliate(
        &self,
        old: &Box<dyn View<Message>>,
        widget_id: WidgetId,
        widget_cx: &mut WidgetCx<Message>,
    ) {
        if self.as_any().downcast_ref::<Length>().unwrap()
            != (**old).as_any().downcast_ref::<Length>().unwrap()
        {
            self.change_widget(widget_cx.widget(widget_id))
        }
    }
}
