use crate::widget::WidgetData;
use crate::AsAny;

pub trait View<Message>: AsAny
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn build_widget(&self) -> WidgetData<Message>;
    fn change_widget(&self, widget: &mut WidgetData<Message>);
    fn reconciliate(&self, old: &Box<dyn View<Message>>, widget: &mut WidgetData<Message>);
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

pub struct ViewSequence<Message>(pub Vec<Box<dyn View<Message>>>);

impl<V, Message> From<Vec<V>> for ViewSequence<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
    V: View<Message>,
{
    fn from(value: Vec<V>) -> Self {
        ViewSequence(
            value
                .into_iter()
                .map(|view| Box::new(view) as Box<dyn View<Message>>)
                .collect(),
        )
    }
}
