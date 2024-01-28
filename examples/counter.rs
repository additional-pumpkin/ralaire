use ralaire::app::App;
use ralaire::row;
use ralaire::widget::{button, empty, text, Widget};
use ralaire_core::{Color, Command};
#[derive(Debug, Clone)]
enum Message {
    IncrementCounter,
    DecrementCounter,
}
#[derive(Clone, PartialEq)]
struct Counter {
    counter: i32,
}
impl App for Counter {
    type Message = Message;

    fn new() -> Self {
        Counter { counter: 0 }
    }

    fn title(&self) -> impl Into<String> {
        "Examples - Counter"
    }

    fn header(&self) -> impl Widget<Self::Message> + 'static {
        empty()
    }

    fn view(&self) -> impl Widget<Self::Message> + 'static {
        row!(
            button(text("increment"))
                .on_press(Message::IncrementCounter)
                .color(Color::PINK)
                .radii(5.),
            text(format!["counter: {}", self.counter]),
            button(text("decrement"))
                .on_press(Message::DecrementCounter)
                .color(Color::LIGHT_BLUE)
                .radii(5.),
        )
        .spacing(30.)
    }

    fn update(&mut self, message: Self::Message) -> Vec<Command<Self::Message>> {
        match message {
            Message::IncrementCounter => self.counter += 1,
            Message::DecrementCounter => self.counter -= 1,
        }
        vec![]
    }
}
fn main() -> ralaire::Result {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    Counter::run()
}
