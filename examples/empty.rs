use ralaire::{
    app::App,
    widget::{empty, Widget},
};
use ralaire_core::Command;

#[derive(Debug, Clone)]
enum Message {}

#[derive(PartialEq, Clone)]
struct Empty;

impl App for Empty {
    type Message = Message;

    fn new() -> Self {
        Empty
    }

    fn title(&self) -> impl Into<String> {
        "Examples - Empty"
    }

    fn header(&self) -> impl Widget<Self::Message> + 'static {
        empty()
    }

    fn view(&self) -> impl Widget<Self::Message> + 'static {
        empty()
    }

    fn update(&mut self, _message: Self::Message) -> Vec<Command<Self::Message>> {
        vec![]
    }
}

fn main() -> ralaire::Result {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    Empty::run()
}
