use crate::view::View;
use crate::widget::{self, WidgetData};

pub fn scroll<Message>(child: impl View<Message>) -> Scroll<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    Scroll::new(Box::new(child))
}

pub struct Scroll<Message> {
    child: Box<dyn View<Message>>,
}

impl<Message> Scroll<Message> {
    pub fn new(child: Box<dyn View<Message>>) -> Self {
        Self { child }
    }
}

impl<Message> View<Message> for Scroll<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn build_widget(&self) -> WidgetData<Message> {
        let child = self.child.build_widget();
        let container = widget::Scroll::new(child);
        WidgetData::new(Box::new(container))
    }

    fn change_widget(&self, _widget: &mut WidgetData<Message>) {}

    fn reconciliate(&self, old: &Box<dyn View<Message>>, widget: &mut WidgetData<Message>) {
        let old = (**old).as_any().downcast_ref::<Scroll<Message>>().unwrap();

        // there is only one child...
        for child in widget.inner.children_mut() {
            self.child.reconciliate(&old.child, child)
        }
    }
}
