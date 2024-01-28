use ralaire::app::App;
use ralaire::widget::{button, container, empty, text, Widget};
use ralaire_core::{Animation, AnimationDirection, Color, Command};
#[derive(Debug, Clone)]
enum Message {
    Tick,
    StartAnimation,
    FinishedAnimation,
}
#[derive(Clone, PartialEq)]
struct ColorAnimation {
    start_color: Color,
    end_color: Color,
    duration: std::time::Duration,
    color: Color,
    animation: Animation,
}
impl App for ColorAnimation {
    type Message = Message;

    fn new() -> Self {
        let duration = std::time::Duration::from_secs(3);
        Self {
            color: Color::PINK,
            start_color: Color::PINK,
            end_color: Color::LIGHT_BLUE,
            duration,
            animation: Animation::new(AnimationDirection::Forward, duration),
        }
    }

    fn title(&self) -> impl Into<String> {
        "Examples - ColorAnimation"
    }

    fn header(&self) -> impl Widget<Self::Message> + 'static {
        empty()
    }

    fn view(&self) -> impl Widget<Self::Message> + 'static {
        container(
            button(text("Animated button"))
                .color(self.color)
                .radii(10.)
                .on_press(Message::StartAnimation),
        )
    }

    fn update(&mut self, message: Self::Message) -> Vec<Command<Self::Message>> {
        match message {
            Message::Tick => {
                self.color = interpolate(self.start_color, self.end_color, self.animation.value());
                vec![]
            }
            Message::StartAnimation => {
                vec![Command::Animate {
                    animation: self.animation.clone(),
                    tick_message: Message::Tick,
                    done_message: Message::FinishedAnimation,
                }]
            }
            Message::FinishedAnimation => {
                self.animation = match &self.animation.direction() {
                    AnimationDirection::Forward => {
                        Animation::new(AnimationDirection::Backward, self.duration)
                    }
                    AnimationDirection::Backward => {
                        Animation::new(AnimationDirection::Forward, self.duration)
                    }
                };
                vec![]
            }
        }
    }
}
fn main() -> ralaire::Result {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    ColorAnimation::run()
}

fn linear_component(u: f64) -> f64 {
    if u < 0.04045 {
        u / 12.92
    } else {
        ((u + 0.055) / 1.055).powf(2.4)
    }
}
fn gamma_component(u: f64) -> f64 {
    if u < 0.0031308 {
        12.92 * u
    } else {
        1.055 * u.powf(1.0 / 2.4) - 0.055
    }
}

fn interpolate(start: Color, end: Color, factor: f64) -> Color {
    Color::rgba(
        gamma_component(
            (linear_component(end.r as f64 / 255.) - linear_component(start.r as f64 / 255.))
                * factor
                + linear_component(start.r as f64 / 255.),
        ),
        gamma_component(
            (linear_component(end.g as f64 / 255.) - linear_component(start.g as f64 / 255.))
                * factor
                + linear_component(start.g as f64 / 255.),
        ),
        gamma_component(
            (linear_component(end.b as f64 / 255.) - linear_component(start.b as f64 / 255.))
                * factor
                + linear_component(start.b as f64 / 255.),
        ),
        gamma_component(
            (linear_component(end.a as f64 / 255.) - linear_component(start.a as f64 / 255.))
                * factor
                + linear_component(start.a as f64 / 255.),
        ),
    )
}
