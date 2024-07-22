use crate::view::View;
use crate::widget::{self, WidgetData};

// TODO: remove this
pub struct WindowControls;

impl<Message> View<Message> for WindowControls
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn build_widget(&self) -> WidgetData<Message> {
        WidgetData::new(Box::new(widget::WindowControls::new()))
    }

    fn change_widget(&self, _widget_data: &mut WidgetData<Message>) {}

    fn reconciliate(&self, _old: &Box<dyn View<Message>>, _widget: &mut WidgetData<Message>) {}
}
