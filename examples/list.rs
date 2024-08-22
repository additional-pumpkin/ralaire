use ralaire::application::Application;
use ralaire::view::View;
use ralaire::view::{button, flex, window};
use ralaire::view_seq;
use ralaire::widget::FlexDirection;
use vello::peniko::Color;
use winit::error::EventLoopError;

fn app_logic(list: &mut Vec<String>) -> impl View<Vec<String>> {
    let actions = flex(
        vec![
            button("add task".to_owned())
                .on_press(|list: &mut Vec<String>| {
                    list.push("item".to_string());
                })
                .color(Color::PINK)
                .radius(5.),
            button("remove task".to_owned())
                .on_press(|list: &mut Vec<String>| {
                    list.pop();
                })
                .color(Color::LIGHT_BLUE)
                .radius(5.),
        ]
        .into(),
    )
    .direction(FlexDirection::Row)
    .spacing(30.);
    println!("number of tasks: {}", list.len());
    let tasks = flex(list.clone().into());
    window(flex(view_seq!(actions, tasks)).spacing(20.)).title("List")
}
fn main() -> Result<(), EventLoopError> {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let app = Application::new(vec![], app_logic);
    app.run()
}
