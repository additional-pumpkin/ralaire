use crate::event::{self, mouse::MouseButton};
use crate::widget::{alignment, Container};
use crate::widget::{Widget, WidgetData, WidgetMarker};
use parley::FontContext;
use vello::peniko::kurbo::{Affine, Point, Rect, RoundedRectRadii, Size};
use vello::peniko::{Color, Fill};

pub struct Button<State> {
    pub(crate) size: Size,
    pub(crate) radii: RoundedRectRadii,
    pub(crate) color: Color,
    pub(crate) on_press: Option<Box<dyn Fn(&mut State) + Send + Sync + 'static>>,
    child: Container<State>,
    hovered: bool,
}

impl<State: 'static> Button<State> {
    pub fn new(
        child: impl Widget<State>,
        size: Size,
        radii: RoundedRectRadii,
        color: Color,
        on_press: Option<Box<dyn Fn(&mut State) + Send + Sync + 'static>>,
    ) -> Self {
        let child = Container::new(
            child,
            alignment::Horizontal::Center,
            alignment::Vertical::Center,
            0.0.into(),
        );
        Self {
            size,
            radii,
            color,
            on_press,
            child,
            hovered: false,
        }
    }
}

impl<State> WidgetMarker for Button<State> {}
impl<State: 'static> Widget<State> for Button<State> {
    fn debug_name(&self) -> &str {
        "button"
    }
    fn paint(&mut self, scene: &mut vello::Scene) {
        scene.fill(
            Fill::NonZero,
            Affine::default(),
            self.color,
            None,
            &Rect::from_origin_size(Point::new(0., 0.), self.size).to_rounded_rect(self.radii),
        );
        if self.hovered {
            scene.fill(
                Fill::NonZero,
                Affine::default(),
                Color::BLACK.multiply_alpha(0.1),
                None,
                &Rect::from_origin_size(Point::new(0., 0.), self.size).to_rounded_rect(self.radii),
            );
        }
        self.child.paint(scene);
    }

    fn children(&self) -> Vec<&WidgetData<State>> {
        self.child.children()
    }

    fn children_mut(&mut self) -> Vec<&mut WidgetData<State>> {
        self.child.children_mut()
    }

    fn layout(&mut self, _: Size, font_context: &mut FontContext) -> Size {
        self.child.layout(self.size, font_context);
        self.size
    }

    fn event(
        &mut self,
        event_context: &mut event::EventContext,
        event: event::WidgetEvent,
        state: &mut State,
    ) -> event::Status {
        if let event::WidgetEvent::Mouse(event::mouse::Event::Press {
            position: _,
            button,
        }) = event
        {
            if button == MouseButton::Left {
                if let Some(on_press) = &self.on_press {
                    (on_press)(state);
                    event_context.state_changed = true;
                }
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
