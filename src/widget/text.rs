use crate::widget::{Constraints, Length, Widget, WidgetCx, WidgetSize};
use parley::{style::FontFamily, FontContext, Layout};
use ralaire_core::{Affine, Color, ParleyBrush, Point, Rect, RenderCx, Size, TextLayout};

#[derive(Debug)]
pub struct Text {
    text: String,
    layout: TextLayout,
}

impl Text {
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
        layout_builder.push_default(&parley::style::StyleProperty::Brush(ParleyBrush::default()));
        layout_builder.push_default(&parley::style::StyleProperty::FontStack(
            parley::style::FontStack::List(&[
                FontFamily::Named("EBGaramond08-Regular"),
                FontFamily::Named("Inter"),
                FontFamily::Named("Noto Sans"),
            ]),
        ));
        // layout_builder.push_default(&parley::style::StyleProperty::FontSize(8.));
        // layout_builder.push_default(&parley::style::StyleProperty::FontStyle(
        // parley::swash::Style::Italic,
        // ));
        // layout_builder.push_default(&parley::style::StyleProperty::FontWeight(
        //     parley::swash::Weight::BOLD,
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

impl<Message> Widget<Message> for Text
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn draw(&self, render_cx: &mut RenderCx) {
        render_cx.draw_text(self.layout.clone());
    }

    fn size_hint(&self) -> WidgetSize {
        let TextLayout::ParleyLayout(layout) = &self.layout;
        WidgetSize {
            width: Length::Fixed(layout.width() as f64),
            height: Length::Fixed(layout.height() as f64),
        }
    }

    fn layout(
        &mut self,
        _widget_cx: &mut WidgetCx<Message>,
        constraints: Constraints,
        font_cx: &mut FontContext,
    ) {
        if constraints.min_size != Size::ZERO {
            self.layout_text(self.text.clone(), constraints.min_size, font_cx);
        } else {
            self.layout_text(self.text.clone(), constraints.max_size, font_cx);
        }
    }

    fn overlay(&self, render_cx: &mut RenderCx) {
        let TextLayout::ParleyLayout(layout) = &self.layout;
        render_cx.fill_shape(
            Affine::default(),
            &Rect::from_origin_size(
                Point::ZERO,
                Size::new(layout.width() as f64, layout.height() as f64),
            ),
            Color::rgba(1., 0., 0., 0.3),
        );
    }
}
