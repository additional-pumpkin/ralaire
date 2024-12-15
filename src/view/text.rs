use crate::view::{View, ViewMarker};
use crate::widget;
impl ViewMarker for String {}
impl<State: 'static> View<State> for String {
    type Element = widget::Text;
    fn build(&self) -> Self::Element {
        widget::Text::new(self.clone())
    }

    fn rebuild(&self, old: &Self, element: &mut Self::Element) {
        if self != old {
            element.set_text(self.clone());
        }
    }

    fn teardown(&self, _: &mut Self::Element) {}
}
