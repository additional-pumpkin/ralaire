use crate::AsAny;

use crate::view::View;
use crate::widget::{BarWidget, WidgetData};
pub struct BarView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    left: Option<Box<dyn View<Message>>>,
    middle: Option<Box<dyn View<Message>>>,
    right: Option<Box<dyn View<Message>>>,
    height: f64,
}
#[allow(dead_code)]
impl<Message> BarView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    pub fn new() -> Self {
        Self {
            left: None,
            middle: None,
            right: None,
            height: 30.,
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

impl<Message> View<Message> for BarView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn build_widget(&self) -> WidgetData<Message> {
        let left = self.left.as_ref().map(|view| view.build_widget());
        let middle = self.middle.as_ref().map(|view| view.build_widget());
        let right = self.right.as_ref().map(|view| view.build_widget());

        WidgetData::new(Box::new(BarWidget::new(left, middle, right, self.height)))
    }

    fn change_widget(&self, _widget: &mut WidgetData<Message>) {}

    fn reconciliate(&self, old: &Box<dyn View<Message>>, widget: &mut WidgetData<Message>) {
        let old = (**old).as_any().downcast_ref::<BarView<Message>>().unwrap();

        let bar = (*widget.widget)
            .as_any_mut()
            .downcast_mut::<BarWidget<Message>>()
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
        widget.change_flags.layout = true; // TODO: figure out when this is needed
    }
}
