use ralaire::app::App;
use ralaire::app::WidgetView;
use ralaire::view::image;
use ralaire::view::window;

fn app_logic(_: &mut ()) -> impl WidgetView<()> {
    window(
        image(include_bytes!("../assets/lain.png").into()),
        "Image".to_owned(),
    )
}
fn main() -> Result<(), winit::error::EventLoopError> {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .init();
    App::new((), app_logic).run()
}
