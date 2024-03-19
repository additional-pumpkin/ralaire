use crate::widget::Container;
use crate::widget::{Constraints, Length, Widget, WidgetCx, WidgetSize};
use parley::FontContext;
use ralaire_core::WidgetId;
use ralaire_core::{
    alignment,
    event::{self, mouse::MouseButton},
    Affine, AppMessage, Color, Point, Rect, RenderCx, RoundedRectRadii, Size,
};

#[derive(Debug)]
pub struct Button<Message> {
    size: Size,
    radii: RoundedRectRadii,
    color: Color,
    on_press: Option<Message>,
    child: Container<Message>,
    hovered: bool,
}

impl<Message> Button<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    pub fn new(child: WidgetId) -> Self {
        let child = Container::new(child)
            .h_align(alignment::Horizontal::Center)
            .v_align(alignment::Vertical::Center);

        Self {
            radii: 0.0.into(),
            size: Size::new(200., 50.),
            color: Color::PINK,
            on_press: None,
            child,
            hovered: false,
        }
    }
    pub fn radii(mut self, radii: impl Into<RoundedRectRadii>) -> Self {
        self.radii = radii.into();
        self
    }
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }
}

impl<Message> Widget<Message> for Button<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn draw(&self, render_cx: &mut RenderCx) {
        // tracing::error!("self.bounds: {:?}", self.child.bounds());
        // tracing::error!("self.color: {:?}", self.color);
        render_cx.fill_shape(
            Affine::default(),
            &Rect::from_origin_size(Point::new(0., 0.), self.size).to_rounded_rect(self.radii),
            self.color,
        );
        if self.hovered {
            render_cx.fill_shape(
                Affine::default(),
                &Rect::from_origin_size(Point::new(0., 0.), self.size).to_rounded_rect(self.radii),
                Color::WHITE.with_alpha_factor(0.2),
            );
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.child.children()
    }

    fn size_hint(&self) -> WidgetSize {
        WidgetSize {
            width: Length::Fixed(self.size.width),
            height: Length::Fixed(self.size.height),
        }
    }

    fn bounds_radii(&self) -> RoundedRectRadii {
        self.radii
    }

    fn layout(
        &mut self,
        widget_cx: &mut WidgetCx<Message>,
        _constraints: Constraints,
        font_cx: &mut FontContext,
    ) {
        self.child.layout(
            widget_cx,
            Constraints {
                min_size: self.size,
                max_size: self.size,
            },
            font_cx,
        );
    }

    fn event(
        &mut self,
        event: event::WidgetEvent,
        event_cx: &mut event::EventCx<AppMessage<Message>>,
    ) -> event::Status {
        match event {
            event::WidgetEvent::Mouse(mouse_event) => match mouse_event {
                event::mouse::Event::Release {
                    position: _,
                    button,
                } => {
                    if button == MouseButton::Left {
                        if let Some(messsage) = &self.on_press {
                            event_cx.add_message(AppMessage::User(messsage.clone()));
                        }
                        event::Status::Captured
                    } else {
                        event::Status::Ignored
                    }
                }
                _ => event::Status::Ignored,
            },
            event::WidgetEvent::Touch(_) => todo!(),
            _ => event::Status::Ignored,
        }
    }
    fn set_hover(&mut self, hover: bool) -> event::Status {
        self.hovered = hover;
        tracing::debug!("Set hovered to {}", hover);
        event::Status::Captured
    }
}
