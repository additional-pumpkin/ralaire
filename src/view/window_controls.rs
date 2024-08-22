use crate::view::View;
use crate::widget::{self, WidgetData};

// TODO: remove this
pub struct WindowControls;

impl<State: 'static> View<State> for WindowControls {
    fn build_widget(&self) -> WidgetData<State> {
        WidgetData::new(Box::new(widget::WindowControls::new()))
    }

    fn change_widget(&self, _widget_data: &mut WidgetData<State>) {}

    fn reconciliate(&self, _old: &Box<dyn View<State>>, _widget: &mut WidgetData<State>) {}
}
