use ralaire::{app::App, view::View};
use ralaire_core::Command;
#[derive(Debug, Clone)]
enum Message {}

struct Empty;

impl App for Empty {
    type Message = Message;

    fn new() -> Self {
        Empty
    }

    fn title(&self) -> impl Into<String> {
        "Examples - Empty"
    }

    fn view(&self) -> impl View<Self::Message> {
        "".to_owned()
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
