use crate::event;
use crate::widget::Widget;
use parley::FontContext;
use std::marker::PhantomData;
use vello::kurbo::{Affine, Size};
use vello_svg::usvg;

pub struct Svg<Message> {
    svg: usvg::Tree,
    size: Size,
    phantom_message: PhantomData<Message>,
}

impl<Message> Svg<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    pub fn new(svg: usvg::Tree) -> Self
    {
        let bounds_size = Size::new(svg.size().width() as f64, svg.size().height() as f64);
        Self {
            svg,
            size: bounds_size,
            phantom_message: PhantomData,
        }
    }
    pub fn set_bounds_size(&mut self, size: Size) {
        self.size = size
    }
}

impl<Message> Widget<Message> for Svg<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn paint(&mut self, scene: &mut vello::Scene) {
        let scale = Affine::scale_non_uniform(
            self.size.width / self.svg.size().width() as f64,
            self.size.height / self.svg.size().height() as f64,
        );
        let svg_fragment = vello_svg::render_tree(&self.svg);
        scene.append(&svg_fragment, Some(scale))
    }

    fn debug_name(&self) -> &str {
        "image"
    }
    fn layout(&mut self, _size_hint: Size, _font_cx: &mut FontContext) -> Size {
        self.size
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
        event::Status::Captured
    }

    fn set_hover(&mut self, _hover: bool) -> event::Status {
        event::Status::Ignored
    }
}
