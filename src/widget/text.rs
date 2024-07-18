use crate::event;
use crate::widget::{Constraints, Widget};
use parley::{style::FontFamily, FontContext, Layout};
use vello::peniko::kurbo::{Affine, Size};
use vello::peniko::{Brush, Color};
pub struct TextWidget {
    text: String,
    layout: Layout<Brush>,
}

impl TextWidget {
    pub fn new(text: String) -> Self {
        Self {
            text: text.clone(),
            layout: Layout::new(),
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
        self.layout = layout;
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
    fn paint(&self, scene: &mut vello::Scene) {
        for line in self.layout.lines() {
            for glyph_run in line.glyph_runs() {
                let mut x = glyph_run.offset();
                let y = glyph_run.baseline();
                let run = glyph_run.run();
                let font = run.font();
                let font_size = run.font_size();
                let style = glyph_run.style();
                let synthesis = run.synthesis();
                let glyph_xform = synthesis
                    .skew()
                    .map(|angle| Affine::skew(angle.to_radians().tan() as f64, 0.0));
                let coords = run
                    .normalized_coords()
                    .iter()
                    .map(|coord| vello::skrifa::instance::NormalizedCoord::from_bits(*coord))
                    .collect::<Vec<_>>();
                scene
                    .draw_glyphs(font)
                    .brush(&style.brush)
                    .glyph_transform(glyph_xform)
                    .font_size(font_size)
                    .normalized_coords(&coords)
                    .draw(
                        vello::peniko::Fill::NonZero,
                        glyph_run.glyphs().map(|glyph| {
                            let gx = x + glyph.x;
                            let gy = y - glyph.y;
                            x += glyph.advance;
                            vello::glyph::Glyph {
                                id: glyph.id as _,
                                x: gx,
                                y: gy,
                            }
                        }),
                    );
            }
        }
    }

    fn layout(&mut self, constraints: Constraints, font_cx: &mut FontContext) -> Size {
        if constraints.min_size != Size::ZERO {
            self.layout_text(self.text.clone(), constraints.min_size, font_cx);
        } else {
            self.layout_text(self.text.clone(), constraints.max_size, font_cx);
        }
        Size::new(self.layout.width() as f64, self.layout.height() as f64)
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
