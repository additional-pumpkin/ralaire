use crate::view::View;
use crate::widget::{self, alignment, WidgetData};
use crate::Padding;

pub fn container<State>(child: impl View<State>) -> Container<State> {
    Container::new(Box::new(child))
}

pub struct Container<State> {
    h_alignment: alignment::Horizontal,
    v_alignment: alignment::Vertical,
    padding: Padding,
    child: Box<dyn View<State>>,
}

impl<State> Container<State> {
    pub fn new(child: Box<dyn View<State>>) -> Self {
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

impl<State: 'static> View<State> for Container<State> {
    fn build_widget(&self) -> WidgetData<State> {
        let child = self.child.build_widget();
        let container =
            widget::Container::new(child, self.h_alignment, self.v_alignment, self.padding);
        WidgetData::new(Box::new(container))
    }

    fn change_widget(&self, widget_data: &mut WidgetData<State>) {
        let container = (*widget_data.inner)
            .as_any_mut()
            .downcast_mut::<widget::Container<State>>()
            .unwrap();
        container.h_alignment = self.h_alignment;
        container.v_alignment = self.v_alignment;
        container.padding = self.padding;
        widget_data.change_flags.needs_layout = true;
    }

    fn reconciliate(&self, old: &Box<dyn View<State>>, widget: &mut WidgetData<State>) {
        let old = (**old).as_any().downcast_ref::<Container<State>>().unwrap();
        if self.h_alignment != old.h_alignment
            || self.v_alignment != old.v_alignment
            || self.padding != old.padding
        {
            self.change_widget(widget)
        }
        // there is only one child...
        for child in widget.inner.children_mut() {
            self.child.reconciliate(&old.child, child)
        }
    }
}
