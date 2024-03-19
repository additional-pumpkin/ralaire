use crate::view::View;
use crate::widget::{ButtonWidget, WidgetData};
use ralaire_core::{Color, RoundedRectRadii, Size};

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
            size: Size::new(300., 100.),
            radii: 0.0.into(),
            color: Color::PINK,
            on_press: None,
            child,
        }
    }
    pub fn radii(mut self, radii: impl Into<RoundedRectRadii>) -> Self {
        self.radii = radii.into();
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
        button.on_press = self.on_press.clone();
        widget_data.change_flags.layout = true;
        widget_data.change_flags.draw = true;
    }

    fn reconciliate(&self, old: &Box<dyn View<Message>>, widget: &mut WidgetData<Message>) {
        let old = (**old)
            .as_any()
            .downcast_ref::<ButtonView<Message>>()
            .unwrap();
        if self.size != old.size || self.color != old.color || self.radii != old.radii
        // || new.on_press != old.on_press
        {
            self.change_widget(widget)
        }
        // there is only one child...
        for child in widget.widget.children_mut() {
            self.child.reconciliate(&old.child, child)
        }
    }
}
