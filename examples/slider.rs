use ralaire::{
    app::{App, WidgetView},
    view::{container, slider, window},
};
use winit::error::EventLoopError;

fn app_logic(state: &mut f64) -> impl WidgetView<f64> {
    window(
        container(slider(*state, |state, value| *state = value)),
        "Slider".to_owned(),
    )
}

fn main() -> Result<(), EventLoopError> {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    App::new(0.5, app_logic).run()
}
