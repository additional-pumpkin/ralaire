use vello::peniko::Color;
use ralaire::{
    app::App,
    view::{button, container, window, View},
    Animation, AnimationDirection, Command,
};
#[derive(Debug, Clone)]
enum Message {
    Tick,
    StartAnimation,
    FinishedAnimation,
}
struct ColorAnimation {
    start_color: Color,
    end_color: Color,
    duration: core::time::Duration,
    color: Color,
    animation: Animation,
}
impl App for ColorAnimation {
    type Message = Message;

    fn new() -> Self {
        let duration = core::time::Duration::from_secs(1);
        Self {
            color: Color::PINK,
            start_color: Color::PINK,
            end_color: Color::LIGHT_BLUE,
            duration,
            animation: Animation::new(AnimationDirection::Forward, duration)
                .with_custom_easing(custom_easing),
        }
    }

    fn view(&self) -> impl View<Self::Message> {
        window(container(
            button("Animated button".to_owned())
                .color(self.color)
                .radius(10.)
                .on_press(Message::StartAnimation),
        ))
        .title("Color Animation")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Tick => {
                self.color = interpolate(self.start_color, self.end_color, self.animation.value());
                Command::none()
            }
            Message::StartAnimation => Command::animation(
                self.animation.clone(),
                Message::Tick,
                Message::FinishedAnimation,
            ),
            Message::FinishedAnimation => {
                self.animation = match &self.animation.direction() {
                    AnimationDirection::Forward => {
                        Animation::new(AnimationDirection::Backward, self.duration)
                            .with_custom_easing(custom_easing)
                    }
                    AnimationDirection::Backward => {
                        Animation::new(AnimationDirection::Forward, self.duration)
                            .with_custom_easing(custom_easing)
                    }
                };
                Command::none()
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

fn custom_easing(value: f64) -> f64 {
    -2. / (1. + value) + 2.
}
