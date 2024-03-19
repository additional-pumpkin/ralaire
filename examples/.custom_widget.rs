use ralaire::{
    app::App,
    widget::{container, empty},
};
use ralaire_core::{
    Affine, Circle, Color, Command, Constraints, Length, Point, Widget, WidgetData, WidgetSize,
};

#[derive(Debug, Clone)]
enum Message {}

#[derive(PartialEq, Clone)]
struct Empty;

impl App for Empty {
    type Message = Message;

    fn new() -> Self {
        Empty
    }

    fn title(&self) -> impl Into<String> {
        "Examples - Custom Widget"
    }

    fn header(&self) -> impl Widget<Self::Message> + 'static {
        empty()
    }

    fn view(&self) -> impl Widget<Self::Message> + 'static {
        container(CustomWidget)
    }

    fn update(&mut self, _message: Self::Message) -> Vec<Command<Self::Message>> {
        vec![]
    }
}

fn main() -> ralaire::Result {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    Empty::run()
}
#[derive(Debug)]
struct CustomWidget;

impl<Message> Widget<Message> for CustomWidget
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn size_hint(&self) -> WidgetSize {
        WidgetSize {
            width: Length::Fixed(200.),
            height: Length::Fixed(200.),
        }
    }

    fn children(&self) -> Vec<&WidgetData<Message>> {
        vec![]
    }

    fn layout(&mut self, _constraints: Constraints, _font_cx: &mut parley::FontContext) {}

    fn draw(&self, render_cx: &mut ralaire_core::RenderCx) {
        render_cx.fill_shape(
            Affine::default(),
            &Circle::new(Point::new(100., 100.), 100.),
            Color::PINK,
        )
    }
}
