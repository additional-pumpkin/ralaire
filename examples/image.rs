use ralaire::view::{image, scroll};
use ralaire::Command;
use ralaire::{
    app::App,
    view::{window, View},
};
#[derive(Debug, Clone)]
enum Message {}

struct Image;

impl App for Image {
    type Message = Message;

    fn new() -> Self {
        Image
    }

    fn view(&self) -> impl View<Self::Message> {
        window(scroll(image(include_bytes!("../assets/paris-30k.svg").into()))).title("Image")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }
}

fn main() -> ralaire::Result {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    Image::run()
}
