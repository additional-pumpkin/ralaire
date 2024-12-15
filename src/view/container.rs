use crate::view::{View, ViewMarker};
use crate::widget::{self, alignment, Widget};
use crate::Padding;

pub fn container<Child>(child: Child) -> Container<Child> {
    Container::new(child)
}

pub struct Container<Child> {
    h_alignment: alignment::Horizontal,
    v_alignment: alignment::Vertical,
    padding: Padding,
    child: Child,
}

impl<Child> Container<Child> {
    pub fn new(child: Child) -> Self {
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
impl<Child> ViewMarker for Container<Child> {}

impl<State: 'static, Child: View<State>> View<State> for Container<Child>
where
    Child::Element: Widget<State>,
{
    type Element = widget::Container<State>;
    fn build(&self) -> Self::Element {
        let child = self.child.build();
        let container =
            widget::Container::new(child, self.h_alignment, self.v_alignment, self.padding);
        container
    }

    fn rebuild(&self, old: &Self, element: &mut Self::Element) {
        if self.h_alignment != old.h_alignment
            || self.v_alignment != old.v_alignment
            || self.padding != old.padding
        {
            element.h_alignment = self.h_alignment;
            element.v_alignment = self.v_alignment;
            element.padding = self.padding;
        }
        // there is only one child...
        for child in element.children_mut() {
            self.child.rebuild(
                &old.child,
                (*child.inner)
                    .as_any_mut()
                    .downcast_mut::<Child::Element>()
                    .unwrap(),
            )
        }
    }

    fn teardown(&self, element: &mut Self::Element) {
        for child in element.children_mut() {
            self.child.teardown(
                (*child.inner)
                    .as_any_mut()
                    .downcast_mut::<Child::Element>()
                    .unwrap(),
            )
        }
    }
}
