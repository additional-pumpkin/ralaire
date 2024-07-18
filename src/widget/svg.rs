use crate::event;
use crate::widget::{Constraints, Widget};
use parley::FontContext;
use vello::peniko::kurbo::Size;
use vello::peniko::BlendMode;
use std::fs;
use std::marker::PhantomData;
use std::path::Path;
use vello::kurbo::{Affine, Shape};
use vello_svg::usvg;

pub struct SvgWidget<Message> {
    svg: usvg::Tree,
    size: Size,
    phantom_message: PhantomData<Message>,
}

impl<Message> SvgWidget<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    pub fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let svg_text = fs::read_to_string(path).unwrap();
        let tree = usvg::Tree::from_str(&svg_text, &usvg::Options::default()).unwrap();
        let bounds_size = Size::new(tree.size().width() as f64, tree.size().height() as f64);
        Self {
            svg: tree,
            size: bounds_size,
            phantom_message: PhantomData,
        }
    }
    pub fn set_bounds_size(&mut self, size: Size) {
        self.size = size
    }
}

impl<Message> Widget<Message> for SvgWidget<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn paint(&self, scene: &mut vello::Scene) {
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
    fn layout(&mut self, _constraints: Constraints, _font_cx: &mut FontContext) -> Size {
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
