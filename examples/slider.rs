use ralaire::{
    application::Application,
    view::{container, slider, window, View},
};
use winit::error::EventLoopError;

fn app_logic(state: &mut f64) -> impl View<f64> {
    window(container(slider(*state, |state, value| *state = value))).title("Text")
}

fn main() -> Result<(), EventLoopError> {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let app = Application::new(0.5, app_logic);
    app.run()
}
