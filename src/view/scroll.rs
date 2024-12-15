// use crate::view::View;
// use crate::widget::{self, Widget, WidgetData};

// pub fn scroll<Child>(child: Child) -> Scroll<Child> {
//     Scroll::new(child)
// }

// pub struct Scroll<Child> {
//     child: Child,
// }

// impl<Child> Scroll<Child> {
//     pub fn new(child: Child) -> Self {
//         Self { child }
//     }
// }

// impl<State: 'static, Child> View<State> for Scroll<Child>
// where
//     Child: View<State>,
// {
//     type Element = widget::Scroll<State>;
//     fn build(&self) -> WidgetData<State> {
//         let child = self.child.build();
//         let container = widget::Scroll::new(child);
//         WidgetData::new(Box::new(container))
//     }

//     fn rebuild(&self, old: &Self, widget: &mut Self::Element) {
//         // there is only one child...
//         for child in widget.children_mut() {
//             self.child.rebuild(
//                 &old.child,
//                 child
//                     .inner
//                     .as_any_mut()
//                     .downcast_mut::<Child::Element>()
//                     .unwrap(),
//             )
//         }
//     }
// }
