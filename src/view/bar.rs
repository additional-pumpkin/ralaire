use crate::AsAny;

use crate::view::View;
use crate::widget::{self, WidgetData};
pub struct Bar<State> {
    left: Option<Box<dyn View<State>>>,
    middle: Option<Box<dyn View<State>>>,
    right: Option<Box<dyn View<State>>>,
    height: f64,
}
#[allow(dead_code)]
impl<State> Bar<State> {
    pub fn new() -> Self {
        Self {
            left: None,
            middle: None,
            right: None,
            height: 30.,
        }
    }
    pub fn left(mut self, view: impl View<State>) -> Self {
        self.left = Some(Box::new(view));
        self
    }
    pub fn middle(mut self, view: impl View<State>) -> Self {
        self.middle = Some(Box::new(view));
        self
    }
    pub fn right(mut self, view: impl View<State>) -> Self {
        self.right = Some(Box::new(view));
        self
    }
}

impl<State: 'static> View<State> for Bar<State> {
    fn build_widget(&self) -> WidgetData<State> {
        let left = self.left.as_ref().map(|view| view.build_widget());
        let middle = self.middle.as_ref().map(|view| view.build_widget());
        let right = self.right.as_ref().map(|view| view.build_widget());

        WidgetData::new(Box::new(widget::Bar::new(left, middle, right, self.height)))
    }

    fn change_widget(&self, _widget: &mut WidgetData<State>) {}

    fn reconciliate(&self, old: &Box<dyn View<State>>, widget: &mut WidgetData<State>) {
        let old = (**old).as_any().downcast_ref::<Bar<State>>().unwrap();

        let bar = (*widget.inner)
            .as_any_mut()
            .downcast_mut::<widget::Bar<State>>()
            .unwrap();
        // left
        if let Some(new_child) = &self.left {
            if let Some(old_child) = &old.left {
                if new_child.as_any().type_id() == old_child.as_any().type_id() {
                    new_child.reconciliate(old_child, bar.left().as_mut().unwrap());
                } else {
                    let new_widget = new_child.build_widget();
                    *bar.left() = Some(new_widget);
                }
            } else {
                let new_widget = new_child.build_widget();
                *bar.left() = Some(new_widget);
            }
        } else {
            *bar.left() = None;
        }

        // middle
        if let Some(new_child) = &self.middle {
            if let Some(old_child) = &old.middle {
                if new_child.as_any().type_id() == old_child.as_any().type_id() {
                    new_child.reconciliate(old_child, bar.middle().as_mut().unwrap());
                } else {
                    let new_widget = new_child.build_widget();
                    *bar.middle() = Some(new_widget);
                }
            } else {
                let new_widget = new_child.build_widget();
                *bar.middle() = Some(new_widget);
            }
        } else {
            *bar.middle() = None;
        }

        // right
        if let Some(new_child) = &self.right {
            if let Some(old_child) = &old.right {
                if new_child.as_any().type_id() == old_child.as_any().type_id() {
                    new_child.reconciliate(old_child, bar.right().as_mut().unwrap());
                } else {
                    let new_widget = new_child.build_widget();
                    *bar.right() = Some(new_widget);
                }
            } else {
                let new_widget = new_child.build_widget();
                *bar.right() = Some(new_widget);
            }
        } else {
            *bar.right() = None;
        }

        bar.set_height(self.height);
        widget.change_flags.needs_layout = true; // TODO: figure out when this is needed
    }
}
