use crate::view::View;
use crate::widget::{WidgetData, WindowControlsWidget};

pub struct WindowControlsView;

impl<Message> View<Message> for WindowControlsView
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn build_widget(&self) -> WidgetData<Message> {
        WidgetData::new(Box::new(WindowControlsWidget::new()))
    }

    fn change_widget(&self, _widget_data: &mut WidgetData<Message>) {}

    fn reconciliate(&self, _old: &Box<dyn View<Message>>, _widget: &mut WidgetData<Message>) {}
}
