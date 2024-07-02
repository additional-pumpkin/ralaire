use crate::{view::View, widget::RootWidget};

pub struct RootView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    child: Box<dyn View<Message>>,
}

impl<Message> RootView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    pub fn new(child: Box<dyn View<Message>>) -> Self {
        Self {  child }
    }

    pub fn build_widget(&self) -> RootWidget<Message> {
        let child = self.child.build_widget();
        RootWidget::new(child)
    }

    pub fn reconciliate(&self, old: &RootView<Message>, root_widget: &mut RootWidget<Message>) {
        if self.child.as_any().type_id() == old.child.as_any().type_id() {
            self.child
                .reconciliate(&old.child, root_widget.child());
        } else {
            let widget = self.child.build_widget();
            *root_widget.child() = widget;
        }
    }
}

