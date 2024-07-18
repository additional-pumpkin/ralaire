use ralaire::{
    app::App,
    view::{container, slider, window, View},
    Command,
};

#[derive(Debug, Clone)]
enum Message {
    SliderChanged(f64),
}

struct Slider {
    value: f64,
}

impl App for Slider {
    type Message = Message;

    fn new() -> Self {
        Self { value: 0.5 }
    }

    fn view(&self) -> impl View<Self::Message> {
        window(container(slider(self.value, Message::SliderChanged))).title("Text")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::SliderChanged(value) => self.value = value,
        }
        Command::none()
    }
}
fn main() -> ralaire::Result {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    Slider::run()
}
