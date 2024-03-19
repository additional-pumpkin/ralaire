use ralaire::view::{button, flex};
use ralaire::view_seq;
use ralaire::widget::FlexDirection;
use ralaire::{app::App, view::View};
use ralaire_core::{Color, Command};
#[derive(Debug, Clone)]
enum Message {
    AddTask,
    RemoveTask,
}
struct Counter {
    tasks: Vec<String>,
}
impl App for Counter {
    type Message = Message;

    fn new() -> Self {
        Counter { tasks: vec![] }
    }

    fn title(&self) -> impl Into<String> {
        "Examples - Counter"
    }

    fn view(&self) -> impl View<Self::Message> {
        let actions = flex(
            vec![
                button("add task".to_owned())
                    .on_press(Message::AddTask)
                    .color(Color::PINK)
                    .radii(5.),
                button("remove task".to_owned())
                    .on_press(Message::RemoveTask)
                    .color(Color::LIGHT_BLUE)
                    .radii(5.),
            ]
            .into(),
        )
        .direction(FlexDirection::Row)
        .spacing(30.);
        println!("number of tasks: {}", self.tasks.len());
        let tasks = flex(self.tasks.clone().into());
        flex(view_seq!(actions, tasks)).spacing(20.)
    }

    fn update(&mut self, message: Self::Message) -> Vec<Command<Self::Message>> {
        match message {
            Message::AddTask => self.tasks.push("task".to_owned()),
            Message::RemoveTask => {
                self.tasks.pop();
            }
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
