mod state;
mod window;
use crate::widget::Widget;
use ralaire_core::Command;
pub use state::AppMessage;
use state::AppState;
pub use state::InternalMessage;
use window::AppWindow;

pub trait App: Sized + Clone + PartialEq {
    type Message: std::fmt::Debug + 'static + Clone + Send;
    fn new() -> Self;
    fn title(&self) -> impl Into<String>;
    fn header(&self) -> impl Widget<Self::Message> + 'static;
    fn view(&self) -> impl Widget<Self::Message> + 'static;
    fn update(&mut self, message: Self::Message) -> Vec<Command<Self::Message>>;
    fn run() -> crate::Result {
        let mut app: AppState<Self::Message> = AppState::new();
        let main_window = app.runner.block_on(AppWindow::<Self::Message>::new(
            &app.event_loop,
            app.title.to_string(),
            &mut app.debug,
        ));
        app.set_main_window(main_window);
        app.run::<Self>()
    }
}
