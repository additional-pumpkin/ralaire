mod state;
mod window;

use crate::view::View;
use ralaire_core::Command;
use state::AppState;
pub trait App: Sized {
    type Message: core::fmt::Debug + 'static + Clone + Send;
    fn new() -> Self;
    fn title(&self) -> impl Into<String>;
    fn view(&self) -> impl View<Self::Message>;
    fn update(&mut self, message: Self::Message) -> Vec<Command<Self::Message>>;
    fn run() -> crate::Result {
        let app: AppState<Self> = AppState::new();
        app.run()
    }
}
