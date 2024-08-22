use crate::AsAny;

use crate::view::View;
use crate::widget::{self, WidgetData};

use super::window_controls::WindowControls;
pub struct Header<State> {
    left: Option<Box<dyn View<State>>>,
    pub(crate) middle: Option<Box<dyn View<State>>>,
    right: Option<Box<dyn View<State>>>,
    window_controls: Box<dyn View<State>>,
}
#[allow(dead_code)]
impl<State: 'static> Header<State> {
    pub fn new() -> Self {
        Self {
            left: None,
            middle: None,
            right: None,
            window_controls: Box::new(WindowControls),
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

impl<State: 'static> View<State> for Header<State> {
    fn build_widget(&self) -> WidgetData<State> {
        let left = self.left.as_ref().map(|view| view.build_widget());
        let middle = self.middle.as_ref().map(|view| view.build_widget());
        let right = self.right.as_ref().map(|view| view.build_widget());
        let window_controls = self.window_controls.build_widget();

        WidgetData::new(Box::new(widget::Header::new(
            left,
            middle,
            right,
            window_controls,
        )))
    }

    fn change_widget(&self, _widget: &mut WidgetData<State>) {}

    fn reconciliate(&self, old: &Box<dyn View<State>>, widget: &mut WidgetData<State>) {
        let old = (**old).as_any().downcast_ref::<Header<State>>().unwrap();

        let header = (*widget.inner)
            .as_any_mut()
            .downcast_mut::<widget::Header<State>>()
            .unwrap();
        // left
        if let Some(new_child) = &self.left {
            if let Some(old_child) = &old.left {
                if new_child.as_any().type_id() == old_child.as_any().type_id() {
                    new_child.reconciliate(old_child, header.left().as_mut().unwrap());
                } else {
                    let new_widget = new_child.build_widget();
                    *header.left() = Some(new_widget);
                }
            } else {
                let new_widget = new_child.build_widget();
                *header.left() = Some(new_widget);
            }
        } else {
            *header.left() = None;
        }

        // middle
        if let Some(new_child) = &self.middle {
            if let Some(old_child) = &old.middle {
                if new_child.as_any().type_id() == old_child.as_any().type_id() {
                    new_child.reconciliate(old_child, header.middle().as_mut().unwrap());
                } else {
                    let new_widget = new_child.build_widget();
                    *header.middle() = Some(new_widget);
                }
            } else {
                let new_widget = new_child.build_widget();
                *header.middle() = Some(new_widget);
            }
        } else {
            *header.middle() = None;
        }

        // right
        if let Some(new_child) = &self.right {
            if let Some(old_child) = &old.right {
                if new_child.as_any().type_id() == old_child.as_any().type_id() {
                    new_child.reconciliate(old_child, header.right().as_mut().unwrap());
                } else {
                    let new_widget = new_child.build_widget();
                    *header.right() = Some(new_widget);
                }
            } else {
                let new_widget = new_child.build_widget();
                *header.right() = Some(new_widget);
            }
        } else {
            *header.right() = None;
        }

        widget.change_flags.needs_layout = true; // TODO: figure out when this is needed
    }
}
