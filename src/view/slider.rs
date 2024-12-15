use crate::view::{View, ViewMarker};
use crate::widget;

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
impl<State> ViewMarker for Slider<State> {}
impl<State: 'static> View<State> for Slider<State> {
    type Element = widget::Slider<State>;
    fn build(&self) -> Self::Element {
        widget::Slider::new(self.value, Box::new(self.on_change.clone()))
    }

    fn rebuild(&self, old: &Self, element: &mut Self::Element) {
        if self.value != old.value || !core::ptr::fn_addr_eq(self.on_change, old.on_change) {
            element.on_change = Box::new(self.on_change.clone());
            element.value = self.value;
        }
    }

    fn teardown(&self, _: &mut Self::Element) {}
}
