use core::f64;

use crate::event;
use crate::widget::{Widget, WidgetMarker};

use parley::FontContext;
use vello::{
    kurbo::{Affine, Circle, Point, Size, Vec2},
    peniko::{BlendMode, Color},
};

use super::WidgetData;

pub struct Scroll<State> {
    size: Size,   // for clipping
    scroll: Vec2, // start = 0.0, end = 1.0
    child: WidgetData<State>,
}

impl<State> Scroll<State> {
    pub fn new(child: WidgetData<State>) -> Self {
        Scroll {
            size: Size::ZERO,
            scroll: Vec2::ZERO,
            child,
        }
    }
}

impl<State> WidgetMarker for Scroll<State> {}
impl<State: 'static> Widget<State> for Scroll<State> {
    fn paint(&mut self, scene: &mut vello::Scene) {
        scene.push_layer(
            BlendMode::default(),
            1.0,
            Affine::default(),
            &self.size.to_rect(),
        );
        self.child.paint(scene);
        scene.pop_layer();
        scene.fill(
            vello::peniko::Fill::NonZero,
            Affine::default(),
            Color::RED,
            None,
            &Circle::new(
                Point::new(self.size.width * -self.scroll.x, self.size.height - 3.),
                4.,
            ),
        );
        dbg!(self.scroll.x);
        dbg!(self.size);
        scene.fill(
            vello::peniko::Fill::NonZero,
            Affine::default(),
            Color::RED,
            None,
            &Circle::new(
                Point::new(self.size.width - 3., self.size.height * -self.scroll.y),
                4.,
            ),
        );
    }
    fn debug_name(&self) -> &str {
        "container"
    }
    fn children(&self) -> Vec<&WidgetData<State>> {
        vec![&self.child]
    }
    fn children_mut(&mut self) -> Vec<&mut WidgetData<State>> {
        vec![&mut self.child]
    }
    fn layout(&mut self, suggested_size: Size, font_context: &mut FontContext) -> Size {
        if !suggested_size.is_finite() {
            panic!("FIXME: size is infinite");
        }
        self.size = suggested_size;
        self.child.size = self
            .child
            .layout(Size::new(f64::INFINITY, f64::INFINITY), font_context);
        self.child.position = Point::new(
            (self.child.size.width - self.size.width) * self.scroll.x,
            (self.child.size.height - self.size.height) * self.scroll.y,
        );
        suggested_size
    }

    fn event(
        &mut self,
        event_context: &mut event::EventContext,
        event: event::WidgetEvent,
        _state: &mut State,
    ) -> event::Status {
        if let event::WidgetEvent::Mouse(event::mouse::Event::Wheel { delta }) = event {
            event_context.repaint_needed = true;
            self.scroll.x += delta.x / (self.child.size.width - self.size.width);
            self.scroll.y += delta.y / (self.child.size.height - self.size.height);
            self.child.position = Point::new(
                (self.child.size.width - self.size.width) * self.scroll.x,
                (self.child.size.height - self.size.height) * self.scroll.y,
            );
            event::Status::Captured
        } else {
            event::Status::Ignored
        }
    }

    fn set_hover(&mut self, _hover: bool) -> event::Status {
        event::Status::Ignored
    }
}
