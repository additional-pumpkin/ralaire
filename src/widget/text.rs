use crate::event;
use crate::renderer::{PaintCx, TextLayout};
use crate::widget::{Constraints, Widget};
use parley::{style::FontFamily, FontContext, Layout};
use peniko::kurbo::Size;
use peniko::{Brush, Color};
pub struct TextWidget {
    text: String,
    layout: TextLayout,
}

impl TextWidget {
    pub fn new(text: String) -> Self {
        Self {
            text: text.clone(),
            layout: TextLayout::ParleyLayout(Layout::new()),
        }
    }

    pub fn layout_text(&mut self, text: String, size: Size, font_cx: &mut FontContext) {
        self.text = text;
        let mut lcx = parley::LayoutContext::new();
        let mut layout_builder = lcx.ranged_builder(font_cx, &self.text, 1.0);
        layout_builder.push_default(&parley::style::StyleProperty::Brush(Brush::Solid(
            Color::BLACK,
        )));
        layout_builder.push_default(&parley::style::StyleProperty::FontStack(
            parley::style::FontStack::List(&[
                // FontFamily::Named("Coromorant Garamont"),
                // FontFamily::Named("Nimbus Roman"),
                FontFamily::Named("Inter"),
                FontFamily::Named("Noto Sans"),
            ]),
        ));
        layout_builder.push_default(&parley::style::StyleProperty::FontSize(14.6666666));
        layout_builder.push_default(&parley::style::StyleProperty::Brush(Brush::Solid(
            Color::BLACK,
        )));
        // layout_builder.push_default(&parley::style::StyleProperty::FontStyle(
        //     parley::style::FontStyle::Italic,
        // ));
        // layout_builder.push_default(&parley::style::StyleProperty::FontWeight(
        //     parley::style::FontWeight::BOLD,
        // ));
        let mut layout = layout_builder.build();
        layout.break_all_lines(Some(size.width as f32), parley::layout::Alignment::Start);
        self.layout = TextLayout::ParleyLayout(layout);
    }
    pub fn text(&self) -> String {
        self.text.clone()
    }
    pub fn set_text(&mut self, text: String) {
        self.text = text
    }
}

impl<Message> Widget<Message> for TextWidget
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn debug_name(&self) -> &str {
        "text"
    }
    fn paint(&self, paint_cx: &mut PaintCx) {
        paint_cx.draw_text(self.layout.clone());
    }

    fn layout(&mut self, constraints: Constraints, font_cx: &mut FontContext) -> Size {
        if constraints.min_size != Size::ZERO {
            self.layout_text(self.text.clone(), constraints.min_size, font_cx);
        } else {
            self.layout_text(self.text.clone(), constraints.max_size, font_cx);
        }
        let TextLayout::ParleyLayout(layout) = &self.layout;
        Size::new(layout.width() as f64, layout.height() as f64)
    }

    fn children(&self) -> Vec<&super::WidgetData<Message>> {
        vec![]
    }

    fn children_mut(&mut self) -> Vec<&mut super::WidgetData<Message>> {
        vec![]
    }

    fn event(
        &mut self,
        _event: event::WidgetEvent,
        _event_cx: &mut event::EventCx<Message>,
    ) -> event::Status {
        event::Status::Ignored
    }

    fn set_hover(&mut self, _hover: bool) -> event::Status {
        event::Status::Ignored
    }
}
