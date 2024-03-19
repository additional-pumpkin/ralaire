use crate::widget::{WidgetCx, WidgetData};
use ralaire_core::{AsAny, WidgetId};

pub trait View<Message>: AsAny
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn build_widget(&self, widget_cx: &mut WidgetCx<Message>) -> WidgetData<Message>;
    fn change_widget(&self, widget: &mut WidgetData<Message>);
    fn reconciliate(
        &self,
        old: &Box<dyn View<Message>>,
        widget_id: WidgetId,
        widget_cx: &mut WidgetCx<Message>,
    );
    fn get_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}
