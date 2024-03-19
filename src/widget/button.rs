use crate::widget::ContainerWidget;
use crate::widget::{Constraints, Length, Widget, WidgetSize};
use parley::FontContext;
use ralaire_core::{
    alignment,
    event::{self, mouse::MouseButton},
    Affine, AppMessage, Color, Point, Rect, RenderCx, RoundedRectRadii, Size,
};

use super::WidgetData;

#[derive(Debug)]
pub struct ButtonWidget<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    pub(crate) size: Size,
    pub(crate) radii: RoundedRectRadii,
    pub(crate) color: Color,
    pub(crate) on_press: Option<Message>,
    child: ContainerWidget<Message>,
    hovered: bool,
}

impl<Message> ButtonWidget<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    pub fn new(
        child: WidgetData<Message>,
        size: Size,
        radii: RoundedRectRadii,
        color: Color,
        on_press: Option<Message>,
    ) -> Self {
        let child = ContainerWidget::new(
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

impl<Message> Widget<Message> for ButtonWidget<Message>
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

    fn children(&self) -> Vec<&WidgetData<Message>> {
        self.child.children()
    }

    fn children_mut(&mut self) -> Vec<&mut WidgetData<Message>> {
        self.child.children_mut()
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

    fn layout(&mut self, _constraints: Constraints, font_cx: &mut FontContext) {
        self.child.layout(
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
                event::mouse::Event::Press {
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
