use crate::{
    view::View,
    widget::{self, WidgetData},
};

use super::{container::Container, Header};

pub fn window<State: 'static>(child: impl View<State>) -> Window<State> {
    Window::new(Box::new(Header::new()), Box::new(child))
}

pub struct Window<State> {
    header: Box<dyn View<State>>,
    content: Box<dyn View<State>>,
    title: String,
}

impl<State: 'static> Window<State> {
    pub fn new(header: Box<dyn View<State>>, content: Box<dyn View<State>>) -> Self {
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
            .downcast_mut::<Header<State>>()
            .unwrap()
            .middle = Some(Box::new(Container::new(Box::new(title.clone()))));
        self.title = title;
        self
    }
}
impl<State: 'static> View<State> for Window<State> {
    fn build_widget(&self) -> WidgetData<State> {
        let header = self.header.build_widget();
        let content = self.content.build_widget();
        WidgetData::new(Box::new(widget::Window::new(
            header,
            content,
            self.title.clone(),
        )))
    }
    fn change_widget(&self, widget: &mut crate::widget::WidgetData<State>) {
        dbg!();
        (*widget.inner)
            .as_any_mut()
            .downcast_mut::<widget::Window<State>>()
            .unwrap()
            .set_title(self.title.clone());
    }
    fn reconciliate(&self, old: &Box<dyn View<State>>, widget: &mut WidgetData<State>) {
        let old = (**old).as_any().downcast_ref::<Window<State>>().unwrap();
        if self.title != old.title {
            self.change_widget(widget)
        }
        let widget = (*widget.inner)
            .as_any_mut()
            .downcast_mut::<widget::Window<State>>()
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
