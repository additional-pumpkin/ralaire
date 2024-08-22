use ralaire::application::Application;
use ralaire::view::View;
use ralaire::view::{button, flex, window};
use ralaire::view_seq;
use ralaire::widget::FlexDirection;
use winit::error::EventLoopError;

fn app_logic(data: &mut i32) -> impl View<i32> {
    window(
        flex(view_seq!(
            button("increment".to_owned())
                .on_press(|data| { *data += 1 })
                .radius(5.),
            if *data % 2 == 0 {
                format!("counter is even: {}", data)
            } else {
                format!("counter is odd: {}", data)
            },
            button("decrement".to_owned())
                .on_press(|data| { *data -= 1 })
                .radius(5.),
        ))
        .direction(FlexDirection::Row)
        .spacing(30.),
    )
    .title("Counter")
}
fn main() -> Result<(), EventLoopError> {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let app = Application::new(0, app_logic);
    app.run()
}
