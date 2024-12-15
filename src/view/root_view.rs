use std::marker::PhantomData;

use crate::{
    view::View,
    widget::{RootWidget, Widget},
};

pub struct RootView<State: 'static, Child>
where
    Child: View<State>,
{
    child: Child,
    phantom_data: PhantomData<State>,
}

impl<State: 'static, Child> RootView<State, Child>
where
    Child: View<State>,
    Child::Element: Widget<State>,
{
    pub fn new(child: Child) -> Self {
        Self {
            child,
            phantom_data: PhantomData,
        }
    }

    pub fn build_widget(&self) -> RootWidget<State> {
        let child = self.child.build();
        RootWidget::new(child)
    }

    pub fn reconciliate(&self, old: &RootView<State, Child>, root_widget: &mut RootWidget<State>) {
        self.child.rebuild(
            &old.child,
            (*root_widget.child().inner)
                .as_any_mut()
                .downcast_mut::<Child::Element>()
                .unwrap(),
        );
    }
    pub fn teardown(&self, root_widget: &mut RootWidget<State>) {
        self.child.teardown(
            (*root_widget.child().inner)
                .as_any_mut()
                .downcast_mut::<Child::Element>()
                .unwrap(),
        );
    }
}
