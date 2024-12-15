// use std::marker::PhantomData;

// use crate::AsAny;

// use crate::view::View;
// use crate::widget::{self, WidgetData};

// pub struct Bar<State, Left, Middle, Right> {
//     left: Option<Left>,
//     pub(crate) middle: Option<Middle>,
//     right: Option<Right>,
//     phantom_data: PhantomData<State>,
// }
// #[allow(dead_code)]
// impl<State: 'static, Left, Middle, Right> Bar<State, Left, Middle, Right> {
//     pub fn new() -> Self {
//         Self {
//             left: None,
//             middle: None,
//             right: None,
//             phantom_data: PhantomData,
//         }
//     }
//     pub fn left(mut self, view: Left) -> Self {
//         self.left = Some(view);
//         self
//     }
//     pub fn middle(mut self, view: Middle) -> Self {
//         self.middle = Some(view);
//         self
//     }
//     pub fn right(mut self, view: Right) -> Self {
//         self.right = Some(view);
//         self
//     }
// }

// impl<State: 'static, Left, Middle, Right> View<State> for Bar<State, Left, Middle, Right>
// where
//     Left: View<State>,
//     Middle: View<State>,
//     Right: View<State>,
// {
//     type Element = widget::Bar<State>;

//     fn build(&self) -> WidgetData<State> {
//         let left = self.left.as_ref().map(|view| view.build());
//         let middle = self.middle.as_ref().map(|view| view.build());
//         let right = self.right.as_ref().map(|view| view.build());

//         WidgetData::new(Box::new(widget::Bar::new(left, middle, right, 30.)))
//     }

//     fn rebuild(&self, old: &Self, element: &mut Self::Element) {
//         let header = element;
//         // left
//         if let Some(new_child) = &self.left {
//             if let Some(old_child) = &old.left {
//                 if new_child.as_any().type_id() == old_child.as_any().type_id() {
//                     new_child.rebuild(
//                         old_child,
//                         header
//                             .left()
//                             .as_mut()
//                             .unwrap()
//                             .inner
//                             .as_any_mut()
//                             .downcast_mut::<Left::Element>()
//                             .unwrap(),
//                     );
//                 } else {
//                     let new_widget = new_child.build();
//                     *header.left() = Some(new_widget);
//                 }
//             } else {
//                 let new_widget = new_child.build();
//                 *header.left() = Some(new_widget);
//             }
//         } else {
//             *header.left() = None;
//         }

//         // middle
//         if let Some(new_child) = &self.middle {
//             if let Some(old_child) = &old.middle {
//                 if new_child.as_any().type_id() == old_child.as_any().type_id() {
//                     new_child.rebuild(
//                         old_child,
//                         header
//                             .middle()
//                             .as_mut()
//                             .unwrap()
//                             .inner
//                             .as_any_mut()
//                             .downcast_mut::<Middle::Element>()
//                             .unwrap(),
//                     );
//                 } else {
//                     let new_widget = new_child.build();
//                     *header.middle() = Some(new_widget);
//                 }
//             } else {
//                 let new_widget = new_child.build();
//                 *header.middle() = Some(new_widget);
//             }
//         } else {
//             *header.middle() = None;
//         }

//         // right
//         if let Some(new_child) = &self.right {
//             if let Some(old_child) = &old.right {
//                 if new_child.as_any().type_id() == old_child.as_any().type_id() {
//                     new_child.rebuild(
//                         old_child,
//                         header
//                             .right()
//                             .as_mut()
//                             .unwrap()
//                             .inner
//                             .as_any_mut()
//                             .downcast_mut::<Right::Element>()
//                             .unwrap(),
//                     );
//                 } else {
//                     let new_widget = new_child.build();
//                     *header.right() = Some(new_widget);
//                 }
//             } else {
//                 let new_widget = new_child.build();
//                 *header.right() = Some(new_widget);
//             }
//         } else {
//             *header.right() = None;
//         }
//     }
// }
