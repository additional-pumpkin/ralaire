use crate::view::View;
use crate::widget::{self, WidgetData};

pub fn slider<State>(value: f64, on_change: fn(&mut State, f64)) -> Slider<State> {
    Slider::new(value, on_change)
}

pub struct Slider<State> {
    value: f64,
    on_change: fn(&mut State, f64),
}

impl<State> Slider<State> {
    pub fn new(value: f64, on_change: fn(&mut State, f64)) -> Self
where {
        Self { value, on_change }
    }
}

impl<State: 'static> View<State> for Slider<State> {
    fn build_widget(&self) -> WidgetData<State> {
        WidgetData::new(Box::new(widget::Slider::new(
            self.value,
            Box::new(self.on_change.clone()),
        )))
    }

    fn change_widget(&self, widget_data: &mut WidgetData<State>) {
        let slider = (*widget_data.inner)
            .as_any_mut()
            .downcast_mut::<widget::Slider<State>>()
            .unwrap();
        slider.on_change = Box::new(self.on_change.clone());
        slider.value = self.value;
        widget_data.change_flags.needs_paint = true;
    }

    fn reconciliate(&self, old: &Box<dyn View<State>>, widget: &mut WidgetData<State>) {
        let old = (**old).as_any().downcast_ref::<Slider<State>>().unwrap();
        if self.value != old.value || self.on_change != old.on_change {
            self.change_widget(widget)
        }
    }
}
