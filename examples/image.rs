use ralaire::application::Application;
use ralaire::view::{image, scroll};
use ralaire::view::{window, View};

fn app_logic(_: &mut ()) -> impl View<()> {
    window(scroll(image(
        include_bytes!("../assets/paris-30k.svg").into(),
    )))
    .title("Image")
}
fn main() -> Result<(), winit::error::EventLoopError> {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let app = Application::new((), app_logic);
    app.run()
}
