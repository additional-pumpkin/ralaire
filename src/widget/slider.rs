use super::WidgetData;
use crate::event::{
    self,
    mouse::{self, MouseButton},
};
use crate::widget::Widget;
use parley::FontContext;
use vello::kurbo::Affine;
use vello::peniko::kurbo::{Circle, Point, Rect, Size};
use vello::peniko::{Color, Fill};

const SLIDER_HEIGHT: f64 = 50.;
pub struct Slider<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    // TODO: Support vertical sliders
    length: f64,
    pub(crate) value: f64, // from 0.0 to 1.0
    pub(crate) on_change: Box<dyn Fn(f64) -> Message>,
    is_dragging: bool,
    hovered: bool,
}

impl<Message> Slider<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    pub fn new(value: f64, on_change: Box<dyn Fn(f64) -> Message>) -> Self {
        Self {
            length: 0.,
            value,
            on_change,
            is_dragging: false,
            hovered: false,
        }
    }
}

impl<Message> Widget<Message> for Slider<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn debug_name(&self) -> &str {
        "slider"
    }
    fn paint(&mut self, scene: &mut vello::Scene) {
        scene.fill(
            Fill::NonZero,
            Affine::default(),
            Color::LIGHT_GRAY,
            None,
            &Rect::from_origin_size(
                Point::new(0., 0.),
                Size::new(self.length, SLIDER_HEIGHT / 2.),
            )
            .to_rounded_rect(SLIDER_HEIGHT / 4.),
        );
        scene.fill(
            Fill::NonZero,
            Affine::default(),
            Color::GRAY,
            None,
            &Circle::new(
                Point::new(self.value * self.length, SLIDER_HEIGHT / 4.),
                SLIDER_HEIGHT / 2.,
            ),
        );
        if self.is_dragging {
            scene.fill(
                Fill::NonZero,
                Affine::default(),
                Color::BLACK.with_alpha_factor(0.3),
                None,
                &Circle::new(
                    Point::new(self.value * self.length, SLIDER_HEIGHT / 4.),
                    SLIDER_HEIGHT / 2.,
                ),
            );
        }
    }

    fn children(&self) -> Vec<&WidgetData<Message>> {
        vec![]
    }

    fn children_mut(&mut self) -> Vec<&mut WidgetData<Message>> {
        vec![]
    }

    fn layout(&mut self, size_hint: Size, _font_cx: &mut FontContext) -> Size {
        self.length = size_hint.width;
        Size::new(self.length, SLIDER_HEIGHT)
    }

    fn event(
        &mut self,
        event: event::WidgetEvent,
        event_cx: &mut event::EventCx<Message>,
    ) -> event::Status {
        if let event::WidgetEvent::Mouse(mouse_event) = event {
            match mouse_event {
                mouse::Event::Move { position } => {
                    if self.is_dragging {
                        // Changing self.value is done by the user
                        event_cx.push_user_message((self.on_change)(position.x / self.length));
                    }
                    return event::Status::Captured;
                }
                mouse::Event::Wheel { delta: _ } => {
                    // TODO: Maybe this should do something? idk
                }
                event::mouse::Event::Press { position, button } => {
                    if button == MouseButton::Left {
                        if position.x > self.value * self.length - SLIDER_HEIGHT / 2.
                            && position.x < self.value * self.length + SLIDER_HEIGHT / 2.
                        {
                            self.is_dragging = true;
                        }
                        return event::Status::Captured;
                    }
                }
                mouse::Event::Release {
                    position: _,
                    button,
                } => {
                    if button == MouseButton::Left {
                        self.is_dragging = false;
                    }
                    return event::Status::Captured;
                }
            }
        }
        event::Status::Ignored
    }
    fn set_hover(&mut self, hover: bool) -> event::Status {
        self.hovered = hover;
        tracing::debug!("Set hovered to {}", hover);
        event::Status::Captured
    }
}
