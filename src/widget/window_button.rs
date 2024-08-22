use super::WidgetData;
use crate::event::{self, mouse::MouseButton};
use crate::widget::alignment;
use crate::widget::Container;
use crate::widget::Widget;
use crate::InternalMessage;
use parley::FontContext;
use vello::kurbo::Affine;
use vello::peniko::kurbo::{Circle, Point, Size};
use vello::peniko::{Color, Fill};

const SIZE: Size = Size::new(24., 24.);
const CENTER: Point = Point::new(SIZE.width / 2., SIZE.height / 2.);
const RADIUS: f64 = SIZE.width / 2.;
pub struct WindowButtonWidget<State> {
    on_press: InternalMessage,
    child: Container<State>,
    hovered: bool,
}

impl<State> WindowButtonWidget<State> {
    pub fn new(child: WidgetData<State>, on_press: InternalMessage) -> Self {
        let child = Container::new(
            child,
            alignment::Horizontal::Center,
            alignment::Vertical::Center,
            0.0.into(),
        );
        Self {
            on_press,
            child,
            hovered: false,
        }
    }
}

impl<State: 'static> Widget<State> for WindowButtonWidget<State> {
    fn debug_name(&self) -> &str {
        "window_button"
    }
    fn paint(&mut self, scene: &mut vello::Scene) {
        scene.fill(
            Fill::NonZero,
            Affine::default(),
            Color::rgb8(235, 235, 235),
            // Color::RED,
            None,
            &Circle::new(CENTER, RADIUS),
        );
        if self.hovered {
            scene.fill(
                Fill::NonZero,
                Affine::default(),
                Color::rgb8(223, 223, 223),
                None,
                &Circle::new(CENTER, RADIUS),
            );
        }
        // scene.push_layer(
        //     BlendMode::new(Mix::Normal, Compose::SrcOver),
        //     1.0,
        //     Affine::default(),
        //     &SIZE.to_rect(),
        // );
        self.child.paint(scene);
        // scene.push_layer(
        //     BlendMode::new(Mix::Normal, Compose::SrcAtop),
        //     1.0,
        //     Affine::default(),
        //     &SIZE.to_rect(),
        // );
        // scene.fill(
        //     Fill::NonZero,
        //     Affine::default(),
        //     Color::WHITE,
        //     None,
        //     &SIZE.to_rect(),
        // );
        // scene.pop_layer();
        // scene.pop_layer();
    }

    fn children(&self) -> Vec<&WidgetData<State>> {
        self.child.children()
    }

    fn children_mut(&mut self) -> Vec<&mut WidgetData<State>> {
        self.child.children_mut()
    }

    fn layout(&mut self, _size_hint: Size, font_cx: &mut FontContext) -> Size {
        self.child.layout(SIZE, font_cx);
        SIZE
    }

    fn event(
        &mut self,
        event_cx: &mut event::EventCx,
        event: event::WidgetEvent,
        state: &mut State,
    ) -> event::Status {
        if let event::WidgetEvent::Mouse(event::mouse::Event::Press {
            position: _,
            button,
        }) = event
        {
            if button == MouseButton::Left {
                event_cx.push_internal_message(self.on_press.clone());
                return event::Status::Captured;
            }
        }
        event::Status::Ignored
    }
    fn set_hover(&mut self, hover: bool) -> event::Status {
        self.hovered = hover;
        event::Status::Captured
    }
}
