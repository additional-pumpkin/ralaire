use crate::{view::View, widget::RootWidget};

pub struct RootView<State> {
    child: Box<dyn View<State>>,
}

impl<State: 'static> RootView<State> {
    pub fn new(child: Box<dyn View<State>>) -> Self {
        Self { child }
    }

    pub fn build_widget(&self) -> RootWidget<State> {
        let child = self.child.build_widget();
        RootWidget::new(child)
    }

    pub fn reconciliate(&self, old: &RootView<State>, root_widget: &mut RootWidget<State>) {
        if self.child.as_any().type_id() == old.child.as_any().type_id() {
            self.child.reconciliate(&old.child, root_widget.child());
        } else {
            let widget = self.child.build_widget();
            *root_widget.child() = widget;
        }
    }
}
