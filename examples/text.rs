use ralaire::{
    app::App,
    view::{container, window, View},
    Command,
};

const LOREM: &str =
    r"Lorem Lorem Lorem LoremLorem LoremLorem LoremLorem LoremLorem LoremLorem Lorem";

#[derive(Debug, Clone)]
enum Message {}

struct Text;

impl App for Text {
    type Message = Message;

    fn new() -> Self {
        Text
    }

    fn view(&self) -> impl View<Self::Message> {
        window(container(LOREM.to_owned()).pad(100.)).title("Text")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }
}
fn main() -> ralaire::Result {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    Text::run()
}
