use crate::view::View;
use crate::widget::{SliderWidget, WidgetData};

pub struct SliderView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    value: f64,
    on_change: fn(f64) -> Message,
}

impl<Message> SliderView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    pub fn new(value: f64, on_change: fn(f64) -> Message) -> Self
where {
        Self { value, on_change }
    }
}

impl<Message> View<Message> for SliderView<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn build_widget(&self) -> WidgetData<Message> {
        WidgetData::new(Box::new(SliderWidget::new(
            self.value,
            Box::new(self.on_change.clone()),
        )))
    }

    fn change_widget(&self, widget_data: &mut WidgetData<Message>) {
        let slider = (*widget_data.widget)
            .as_any_mut()
            .downcast_mut::<SliderWidget<Message>>()
            .unwrap();
        slider.on_change = Box::new(self.on_change.clone());
        slider.value = self.value;
        widget_data.change_flags.needs_repaint = true;
    }

    fn reconciliate(&self, old: &Box<dyn View<Message>>, widget: &mut WidgetData<Message>) {
        let old = (**old)
            .as_any()
            .downcast_ref::<SliderView<Message>>()
            .unwrap();
        if self.value != old.value || self.on_change != old.on_change {
            self.change_widget(widget)
        }
    }
}
