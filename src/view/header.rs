use crate::AsAny;

use crate::view::View;
use crate::widget::{HeaderWidget, WidgetData};

use super::window_controls::WindowControlsView;
pub struct HeaderView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    left: Option<Box<dyn View<Message>>>,
    pub(crate) middle: Option<Box<dyn View<Message>>>,
    right: Option<Box<dyn View<Message>>>,
    window_controls: Box<dyn View<Message>>,
}
#[allow(dead_code)]
impl<Message> HeaderView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    pub fn new() -> Self {
        Self {
            left: None,
            middle: None,
            right: None,
            window_controls: Box::new(WindowControlsView),
        }
    }
    pub fn left(mut self, view: impl View<Message>) -> Self {
        self.left = Some(Box::new(view));
        self
    }
    pub fn middle(mut self, view: impl View<Message>) -> Self {
        self.middle = Some(Box::new(view));
        self
    }
    pub fn right(mut self, view: impl View<Message>) -> Self {
        self.right = Some(Box::new(view));
        self
    }
}

impl<Message> View<Message> for HeaderView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn build_widget(&self) -> WidgetData<Message> {
        let left = self.left.as_ref().map(|view| view.build_widget());
        let middle = self.middle.as_ref().map(|view| view.build_widget());
        let right = self.right.as_ref().map(|view| view.build_widget());
        let window_controls = self.window_controls.build_widget();

        WidgetData::new(Box::new(HeaderWidget::new(
            left,
            middle,
            right,
            window_controls,
        )))
    }

    fn change_widget(&self, _widget: &mut WidgetData<Message>) {}

    fn reconciliate(&self, old: &Box<dyn View<Message>>, widget: &mut WidgetData<Message>) {
        let old = (**old)
            .as_any()
            .downcast_ref::<HeaderView<Message>>()
            .unwrap();

        let header = (*widget.widget)
            .as_any_mut()
            .downcast_mut::<HeaderWidget<Message>>()
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
