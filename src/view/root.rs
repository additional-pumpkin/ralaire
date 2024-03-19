use crate::{view::View, widget::RootWidget};

pub struct RootView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    content: Box<dyn View<Message>>,
}

impl<Message> RootView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    pub fn new(content: Box<dyn View<Message>>) -> Self {
        Self { content }
    }

    pub fn build_widget(&self) -> RootWidget<Message> {
        let content_data = self.content.build_widget();
        RootWidget::new(content_data)
    }

    pub fn reconciliate(&self, old: &RootView<Message>, root_widget: &mut RootWidget<Message>) {
        if self.content.as_any().type_id() == old.content.as_any().type_id() {
            self.content
                .reconciliate(&old.content, root_widget.content());
        } else {
            let widget = self.content.build_widget();
            *root_widget.content() = widget;
        }
    }
}
