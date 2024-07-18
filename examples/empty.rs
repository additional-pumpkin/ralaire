use ralaire::{
    app::App,
    view::{window, View},
    Command,
};

#[derive(Debug, Clone)]
enum Message {}

struct Empty;

impl App for Empty {
    type Message = Message;

    fn new() -> Self {
        Empty
    }

    fn view(&self) -> impl View<Self::Message> {
        window("".to_owned()).title("Empty")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }
}

fn main() -> ralaire::Result {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    Empty::run()
}
