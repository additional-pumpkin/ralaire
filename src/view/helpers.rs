// #[macro_export]
// macro_rules! bar {
//     ($left:expr, $middle:expr, $right:expr) => {
//         $crate::view::Bar {
//             left: $left.map(|view| Box::new(view) as Box<dyn $crate::view::View<Message>>),
//             middle: $middle.map(|view| Box::new(view) as Box<dyn $crate::view::View<Message>>),
//             right: $right.map(|view| Box::new(view) as Box<dyn $crate::view::View<Message>>),
//         }
//     };
// }
// pub use bar;

#[macro_export]
macro_rules! row {
    ($($x:expr),+ $(,)?) => {$crate::view::FlexBoxView{
        children: vec![$(ralaire_core::WidgetData::new($x)),+],
        flex_direction: FlexDirection::Row,
    }};
}

pub use row;

#[macro_export]
macro_rules! column {
    ($($x:expr),+ $(,)?) => {$crate::view::FlexBoxView{
        children: vec![$(Box::new($x)),+],
        flex_direction: $crate::widget::FlexDirection::Column,
    }};
}

pub use column;
// #[macro_export]
// macro_rules! view_custom {
//     () => {$crate::Column::new()};
//     ($($x:expr),+ $(,)?) => {$crate::widget::Column::new(vec![$(ralaire_core::WidgetData::new($x)),+])};
// }
