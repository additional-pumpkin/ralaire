use crate::widget::WidgetData;
use crate::AsAny;

pub trait View<State>: AsAny {
    fn build_widget(&self) -> WidgetData<State>;
    fn change_widget(&self, widget: &mut WidgetData<State>);
    fn reconciliate(&self, old: &Box<dyn View<State>>, widget: &mut WidgetData<State>);
    fn get_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

#[macro_export]
macro_rules! view_seq {
    ($($x:expr),+ $(,)?) => {$crate::view::ViewSequence(
        vec![$(Box::new($x)),+],
    )};
}

pub struct ViewSequence<State>(pub Vec<Box<dyn View<State>>>);

impl<V, State> From<Vec<V>> for ViewSequence<State>
where
    V: View<State>,
{
    fn from(value: Vec<V>) -> Self {
        ViewSequence(
            value
                .into_iter()
                .map(|view| Box::new(view) as Box<dyn View<State>>)
                .collect(),
        )
    }
}
