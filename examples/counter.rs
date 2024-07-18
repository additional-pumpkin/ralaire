use ralaire::view::{button, container, flex, window};
use ralaire::view_seq;
use ralaire::widget::FlexDirection;
use ralaire::{app::App, view::View, Command};
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

    fn view(&self) -> impl View<Self::Message> {
        window(
            flex(view_seq!(
                button("increment".to_owned())
                    .on_press(Message::IncrementCounter)
                    .radius(5.),
                format!["counter: {}", self.counter],
                button("decrement".to_owned())
                    .on_press(Message::DecrementCounter)
                    .radius(5.),
            ))
            .direction(FlexDirection::Row)
            .spacing(30.),
        )
        .title("Counter")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::IncrementCounter => self.counter += 1,
            Message::DecrementCounter => self.counter -= 1,
        }
        Command::none()
    }
}
fn main() -> ralaire::Result {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    Counter::run()
}
