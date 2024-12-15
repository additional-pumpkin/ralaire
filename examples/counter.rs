use ralaire::app::{App, WidgetView};
use ralaire::view::window;
use ralaire::view::{button, flex};
use ralaire::widget::JustifyContent;
use winit::error::EventLoopError;

fn app_logic(state: &mut i32) -> impl WidgetView<i32> {
    tracing::warn!("check?");
    window(
        flex((
            button("increment".to_owned())
                .on_press(|state| *state += 1)
                .radius(5.),
            format!("{}", state),
            button("decrement".to_owned())
                .on_press(|state| *state -= 1)
                .radius(5.),
        ))
        .direction(ralaire::widget::FlexDirection::Row)
        .cross_axis_alignment(ralaire::widget::CrossAxisAlignment::Center)
        .justify_content(JustifyContent::SpaceEvenly),
        "Counter".to_owned(),
    )
}
fn main() -> Result<(), EventLoopError> {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    App::new(0, app_logic).run()
}
