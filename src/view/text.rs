use crate::view::View;
use crate::widget::{self, WidgetData};
impl<State: 'static> View<State> for String {
    fn build_widget(&self) -> WidgetData<State> {
        WidgetData::new(Box::new(widget::Text::new(self.clone())))
    }

    fn change_widget(&self, widget: &mut WidgetData<State>) {
        (*widget.inner)
            .as_any_mut()
            .downcast_mut::<widget::Text>()
            .unwrap()
            .set_text(self.clone());
        widget.change_flags.needs_layout = true;
        widget.change_flags.needs_paint = true;
    }

    fn reconciliate(&self, old: &Box<dyn View<State>>, widget: &mut WidgetData<State>) {
        if self != (**old).as_any().downcast_ref::<String>().unwrap() {
            self.change_widget(widget)
        }
    }
}
