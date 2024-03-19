use ralaire::view::{button, flex};
use ralaire::view_seq;
use ralaire::{app::App, view::View};
use ralaire_core::{Color, Command};
#[derive(Debug, Clone)]
enum Message {
    IncrementCounter,
    DecrementCounter,
}
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

    fn view(&self) -> impl View<Self::Message> {
        flex(view_seq!(
            button("increment".to_owned())
                .on_press(Message::IncrementCounter)
                .color(Color::PINK)
                .radii(5.),
            format!["counter: {}", self.counter],
            button("decrement".to_owned())
                .on_press(Message::DecrementCounter)
                .color(Color::LIGHT_BLUE)
                .radii(5.),
        ))
        .spacing(30.)
    }

    fn update(&mut self, message: Self::Message) -> Vec<Command<Self::Message>> {
        match message {
            Message::IncrementCounter => self.counter += rand::random::<i32>().abs() >> 20,
            Message::DecrementCounter => self.counter -= rand::random::<i32>().abs() >> 20,
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
