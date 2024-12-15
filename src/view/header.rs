use crate::view::{View, ViewMarker, WindowControls};
use crate::widget::{self, Widget};

pub struct Header<Left, Middle, Right> {
    left: Left,
    middle: Middle,
    right: Right,
    window_controls: WindowControls,
}
impl<Left, Middle, Right> Header<Left, Middle, Right> {
    pub fn new(left: Left, middle: Middle, right: Right) -> Self {
        Self {
            left,
            middle,
            right,
            window_controls: WindowControls,
        }
    }
}
impl<Left, Middle, Right> ViewMarker for Header<Left, Middle, Right> {}
impl<State: 'static, Left, Middle, Right> View<State> for Header<Left, Middle, Right>
where
    Left: View<State>,
    Middle: View<State>,
    Right: View<State>,
    Left::Element: Widget<State>,
    Middle::Element: Widget<State>,
    Right::Element: Widget<State>,
{
    type Element = widget::Header<State>;

    fn build(&self) -> Self::Element {
        let left = self.left.build();
        let middle = self.middle.build();
        let right = self.right.build();
        let window_controls = self.window_controls.build();

        widget::Header::new(left, middle, right, window_controls)
    }

    fn rebuild(&self, old: &Self, element: &mut Self::Element) {
        self.left.rebuild(
            &old.left,
            (*element.left.inner)
                .as_any_mut()
                .downcast_mut::<Left::Element>()
                .unwrap(),
        );
        self.middle.rebuild(
            &old.middle,
            (*element.middle.inner)
                .as_any_mut()
                .downcast_mut::<Middle::Element>()
                .unwrap(),
        );
        self.right.rebuild(
            &old.right,
            (*element.right.inner)
                .as_any_mut()
                .downcast_mut::<Right::Element>()
                .unwrap(),
        );
        self.window_controls.rebuild(
            &old.window_controls,
            (*element.window_controls.inner)
                .as_any_mut()
                .downcast_mut::<widget::WindowControls<State>>()
                .unwrap(),
        );
    }

    fn teardown(&self, element: &mut Self::Element) {
        self.left.teardown(
            (*element.left.inner)
                .as_any_mut()
                .downcast_mut::<Left::Element>()
                .unwrap(),
        );
        self.middle.teardown(
            (*element.middle.inner)
                .as_any_mut()
                .downcast_mut::<Middle::Element>()
                .unwrap(),
        );
        self.right.teardown(
            (*element.right.inner)
                .as_any_mut()
                .downcast_mut::<Right::Element>()
                .unwrap(),
        );
        self.window_controls.teardown(
            (*element.window_controls.inner)
                .as_any_mut()
                .downcast_mut::<widget::WindowControls<State>>()
                .unwrap(),
        );
    }
}
