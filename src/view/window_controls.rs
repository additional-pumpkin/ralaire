use crate::view::{View, ViewMarker};
use crate::widget;

// TODO: remove this
impl ViewMarker for WindowControls {}
pub struct WindowControls;

impl<State: 'static> View<State> for WindowControls {
    type Element = widget::WindowControls<State>;
    fn build(&self) -> Self::Element {
        widget::WindowControls::new()
    }

    fn rebuild(&self, _: &Self, _: &mut Self::Element) {}

    fn teardown(&self, _: &mut Self::Element) {}
}
