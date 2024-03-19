use crate::{
    view::View,
    widget::{BarWidget, RootWidget, WidgetCx},
};

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

    pub fn build_widget(&self, widget_cx: &mut WidgetCx<Message>) -> RootWidget<Message> {
        let content_data = self.content.build_widget(widget_cx);
        let content = content_data.id;
        widget_cx.add_widget(content_data);
        RootWidget::new(content)
    }

    pub fn reconciliate(
        &self,
        old: &RootView<Message>,
        root_widget: &mut RootWidget<Message>,
        widget_cx: &mut WidgetCx<Message>,
    ) {
        let mut top_bar = BarWidget {
            left: None,
            middle: None,
            right: None,
        };
        if self.content.as_any().type_id() == old.content.as_any().type_id() {
            let widget_id = root_widget.content();
            self.content
                .reconciliate(&old.content, widget_id, widget_cx);
        } else {
            let widget_id = root_widget.content();
            widget_cx.remove_widget(widget_id);
            let widget = self.content.build_widget(widget_cx);
            top_bar.left = Some(widget.id);
            widget_cx.add_widget(widget);
        }
    }
}
