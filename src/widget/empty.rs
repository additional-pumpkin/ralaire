use crate::widget::{Widget, WidgetData};
use parley::FontContext;
use ralaire_core::RenderCx;

use super::widget::{Constraints, Length, WidgetSize};

#[derive(Debug)]
pub struct Empty;

impl Empty {
    pub fn new() -> Self {
        Self
    }
}

impl<Message> Widget<Message> for Empty
where
    Message: std::fmt::Debug + Clone,
{
    fn draw(&self, _render_cx: &mut RenderCx) {}

    fn size_hint(&self) -> WidgetSize {
        WidgetSize {
            width: Length::Flexible(1),
            height: Length::Flexible(1),
        }
    }

    fn layout(&mut self, _constraints: Constraints, _font_cx: &mut FontContext) {}

    fn children(&self) -> Vec<&WidgetData<Message>> {
        vec![]
    }
    fn children_mut(&mut self) -> Vec<&mut WidgetData<Message>> {
        vec![]
    }
}
