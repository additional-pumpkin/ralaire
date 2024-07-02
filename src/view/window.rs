use crate::{
    view::View,
    widget::{WidgetData, WindowWidget},
};

use super::{container::ContainerView, HeaderView};

pub struct WindowView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    header: Box<dyn View<Message>>,
    content: Box<dyn View<Message>>,
    title: String,
}

impl<Message> WindowView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    pub fn new(header: Box<dyn View<Message>>, content: Box<dyn View<Message>>) -> Self {
        Self {
            header,
            content,
            title: String::default(),
        }
    }
    pub fn title(mut self, title: impl Into<String>) -> Self {
        let title = title.into();
        self.header
            .as_any_mut()
            .downcast_mut::<HeaderView<Message>>()
            .unwrap()
            .middle = Some(Box::new(ContainerView::new(Box::new(title.clone()))));
        self.title = title;
        self
    }
}
impl<Message> View<Message> for WindowView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn build_widget(&self) -> WidgetData<Message> {
        let header = self.header.build_widget();
        let content = self.content.build_widget();
        WidgetData::new(Box::new(WindowWidget::new(
            header,
            content,
            self.title.clone(),
        )))
    }
    fn change_widget(&self, widget: &mut crate::widget::WidgetData<Message>) {
        dbg!();
        (*widget.widget)
            .as_any_mut()
            .downcast_mut::<WindowWidget<Message>>()
            .unwrap()
            .set_title(self.title.clone());
    }
    fn reconciliate(&self, old: &Box<dyn View<Message>>, widget: &mut WidgetData<Message>) {
        let old = (**old)
            .as_any()
            .downcast_ref::<WindowView<Message>>()
            .unwrap();
        if self.title != old.title {
            self.change_widget(widget)
        }
        let widget = (*widget.widget)
            .as_any_mut()
            .downcast_mut::<WindowWidget<Message>>()
            .unwrap();
        self.header.reconciliate(&old.header, widget.header());
        if self.content.as_any().type_id() == old.content.as_any().type_id() {
            self.content.reconciliate(&old.content, widget.content());
        } else {
            let new_content = self.content.build_widget();
            *widget.content() = new_content;
        }
    }
}
