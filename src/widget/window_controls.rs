use crate::widget::{Constraints, Widget, WidgetSize};
use parley::FontContext;
use ralaire_core::{Affine, Color, Point, Rect, Size};
use std::marker::PhantomData;

use super::Length;

#[derive(Debug)]
pub struct WindowControlsWidget<Message> {
    phantom_message: PhantomData<Message>,
}

impl<Message> WindowControlsWidget<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    pub fn new() -> Self {
        Self {
            phantom_message: PhantomData,
        }
    }
}

impl<Message> Widget<Message> for WindowControlsWidget<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn size_hint(&self) -> WidgetSize {
        WidgetSize {
            width: Length::Fixed(200.),
            height: Length::Fixed(32.),
        }
    }

    fn layout(&mut self, _constraints: Constraints, _font_cx: &mut FontContext) {}
    fn draw(&self, render_cx: &mut ralaire_core::RenderCx) {
        render_cx.fill_shape(
            Affine::default(),
            &Rect::from_origin_size(Point::new(0., 0.), Size::new(200., 32.)),
            Color::RED,
        )
    }
    fn children(&self) -> Vec<&super::WidgetData<Message>> {
        vec![]
    }
    fn children_mut(&mut self) -> Vec<&mut super::WidgetData<Message>> {
        vec![]
    }
}
