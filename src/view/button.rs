use crate::view::View;
use crate::widget::{ButtonWidget, WidgetData};
use peniko::kurbo::{RoundedRectRadii, Size};
use peniko::Color;
pub struct ButtonView<Message> {
    size: Size,
    radii: RoundedRectRadii,
    color: Color,
    on_press: Option<Message>,
    child: Box<dyn View<Message>>,
}

impl<Message> ButtonView<Message> {
    pub fn new(child: Box<dyn View<Message>>) -> Self {
        Self {
            size: Size::new(60. * 1.5, 23. * 1.5),
            radii: 10.0.into(),
            color: Color::LIGHT_GRAY,
            on_press: None,
            child,
        }
    }
    pub fn radius(mut self, radius: f64) -> Self {
        self.radii = radius.into();
        self
    }
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }
}

impl<Message> View<Message> for ButtonView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn build_widget(&self) -> WidgetData<Message> {
        let child = self.child.build_widget();
        let button = ButtonWidget::new(
            child,
            self.size,
            self.radii,
            self.color,
            self.on_press.clone(),
        );
        WidgetData::new(Box::new(button))
    }

    fn change_widget(&self, widget_data: &mut WidgetData<Message>) {
        let button = (*widget_data.widget)
            .as_any_mut()
            .downcast_mut::<ButtonWidget<Message>>()
            .unwrap();
        button.size = self.size;
        button.color = self.color;
        button.radii = self.radii;
        button.on_press.clone_from(&self.on_press);
        widget_data.change_flags.layout = true;
        widget_data.change_flags.draw = true;
    }

    fn reconciliate(&self, old: &Box<dyn View<Message>>, widget: &mut WidgetData<Message>) {
        let old = (**old)
            .as_any()
            .downcast_ref::<ButtonView<Message>>()
            .unwrap();
        if self.size != old.size || self.color != old.color || self.radii != old.radii
        // || self.on_press != old.on_press
        {
            self.change_widget(widget)
        }
        // there is only one child...
        for child in widget.widget.children_mut() {
            self.child.reconciliate(&old.child, child)
        }
    }
}
