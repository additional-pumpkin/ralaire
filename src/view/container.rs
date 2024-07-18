use crate::view::View;
use crate::widget::{ContainerWidget, WidgetData};
use crate::{alignment, Padding};

pub struct ContainerView<Message> {
    h_alignment: alignment::Horizontal,
    v_alignment: alignment::Vertical,
    padding: Padding,
    child: Box<dyn View<Message>>,
}

impl<Message> ContainerView<Message> {
    pub fn new(child: Box<dyn View<Message>>) -> Self {
        Self {
            h_alignment: alignment::Horizontal::Center,
            v_alignment: alignment::Vertical::Center,
            padding: Padding::ZERO,
            child,
        }
    }
    pub fn pad<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }
    pub fn h_align(mut self, h_alignment: alignment::Horizontal) -> Self {
        self.h_alignment = h_alignment;
        self
    }
    pub fn v_align(mut self, v_alignment: alignment::Vertical) -> Self {
        self.v_alignment = v_alignment;
        self
    }
}

impl<Message> View<Message> for ContainerView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn build_widget(&self) -> WidgetData<Message> {
        let child = self.child.build_widget();
        let container =
            ContainerWidget::new(child, self.h_alignment, self.v_alignment, self.padding);
        WidgetData::new(Box::new(container))
    }

    fn change_widget(&self, widget_data: &mut WidgetData<Message>) {
        let container = (*widget_data.widget)
            .as_any_mut()
            .downcast_mut::<ContainerWidget<Message>>()
            .unwrap();
        container.h_alignment = self.h_alignment;
        container.v_alignment = self.v_alignment;
        container.padding = self.padding;
        widget_data.change_flags.needs_layout = true;
    }

    fn reconciliate(&self, old: &Box<dyn View<Message>>, widget: &mut WidgetData<Message>) {
        let old = (**old)
            .as_any()
            .downcast_ref::<ContainerView<Message>>()
            .unwrap();
        if self.h_alignment != old.h_alignment
            || self.v_alignment != old.v_alignment
            || self.padding != old.padding
        {
            self.change_widget(widget)
        }
        // there is only one child...
        for child in widget.widget.children_mut() {
            self.child.reconciliate(&old.child, child)
        }
    }
}
