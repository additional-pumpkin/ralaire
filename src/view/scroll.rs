use crate::view::View;
use crate::widget::{self, WidgetData};

pub fn scroll<State>(child: impl View<State>) -> Scroll<State> {
    Scroll::new(Box::new(child))
}

pub struct Scroll<State> {
    child: Box<dyn View<State>>,
}

impl<State> Scroll<State> {
    pub fn new(child: Box<dyn View<State>>) -> Self {
        Self { child }
    }
}

impl<State: 'static> View<State> for Scroll<State> {
    fn build_widget(&self) -> WidgetData<State> {
        let child = self.child.build_widget();
        let container = widget::Scroll::new(child);
        WidgetData::new(Box::new(container))
    }

    fn change_widget(&self, _widget: &mut WidgetData<State>) {}

    fn reconciliate(&self, old: &Box<dyn View<State>>, widget: &mut WidgetData<State>) {
        let old = (**old).as_any().downcast_ref::<Scroll<State>>().unwrap();

        // there is only one child...
        for child in widget.inner.children_mut() {
            self.child.reconciliate(&old.child, child)
        }
    }
}
