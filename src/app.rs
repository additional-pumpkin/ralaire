#[derive(Debug, Clone)]
pub enum InternalMessage {
    TitleChanged(String),
    MinimiseWindow,
    MaximiseWindow,
    CloseWindow,
    DragResizeWindow(ResizeDirection),
    DragMoveWindow,
}

mod state;
mod window;

use super::Command;
use crate::view::View;
use state::AppState;
use winit::window::ResizeDirection;
pub trait App: Sized {
    type Message: core::fmt::Debug + 'static + Clone + Send;
    fn new() -> Self;
    fn view(&self) -> impl View<Self::Message>;
    fn update(&mut self, message: Self::Message) -> Command<Self::Message>;
    fn run() -> crate::Result {
        let app: AppState<Self> = AppState::new();
        app.run()
    }
}
