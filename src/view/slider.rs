use crate::view::View;
use crate::widget::{self, WidgetData};

pub fn slider<Message>(value: f64, on_change: fn(f64) -> Message) -> Slider<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    Slider::new(value, on_change)
}

pub struct Slider<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    value: f64,
    on_change: fn(f64) -> Message,
}

impl<Message> Slider<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    pub fn new(value: f64, on_change: fn(f64) -> Message) -> Self
where {
        Self { value, on_change }
    }
}

impl<Message> View<Message> for Slider<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn build_widget(&self) -> WidgetData<Message> {
        WidgetData::new(Box::new(widget::Slider::new(
            self.value,
            Box::new(self.on_change.clone()),
        )))
    }

    fn change_widget(&self, widget_data: &mut WidgetData<Message>) {
        let slider = (*widget_data.inner)
            .as_any_mut()
            .downcast_mut::<widget::Slider<Message>>()
            .unwrap();
        slider.on_change = Box::new(self.on_change.clone());
        slider.value = self.value;
        widget_data.change_flags.needs_paint = true;
    }

    fn reconciliate(&self, old: &Box<dyn View<Message>>, widget: &mut WidgetData<Message>) {
        let old = (**old).as_any().downcast_ref::<Slider<Message>>().unwrap();
        if self.value != old.value || self.on_change != old.on_change {
            self.change_widget(widget)
        }
    }
}
