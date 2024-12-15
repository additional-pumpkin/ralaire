use ralaire::app::{App, WidgetView};
use ralaire::view::{button, flex, window};
use ralaire::widget::FlexDirection;
use winit::error::EventLoopError;

fn app_logic(state: &mut Vec<String>) -> impl WidgetView<Vec<String>> {
    let actions = flex((
        button("add task".to_owned()).on_press(|state: &mut Vec<String>| {
            state.push("item".to_string());
        }),
        button("remove task".to_owned()).on_press(|state: &mut Vec<String>| {
            state.pop();
        }),
    ))
    .direction(FlexDirection::Row);
    println!("number of tasks: {}", state.len());
    let tasks = flex(state.clone());
    window(
        flex((actions, tasks)).cross_axis_alignment(ralaire::widget::CrossAxisAlignment::Center),
        "List".to_owned(),
    )
}
fn main() -> Result<(), EventLoopError> {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    App::new(vec![], app_logic).run()
}
