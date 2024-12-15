use crate::event;
use crate::widget::{Widget, WidgetMarker};
use parley::FontContext;
use vello::kurbo::{Affine, Size};
use vello_svg::usvg;

pub struct Svg {
    svg: usvg::Tree,
    size: Size,
}

impl Svg {
    pub fn new(svg: usvg::Tree) -> Self {
        let bounds_size = Size::new(svg.size().width() as f64, svg.size().height() as f64);
        Self {
            svg,
            size: bounds_size,
        }
    }
    pub fn set_bounds_size(&mut self, size: Size) {
        self.size = size
    }
}

impl WidgetMarker for Svg {}
impl<State: 'static> Widget<State> for Svg {
    fn paint(&mut self, scene: &mut vello::Scene) {
        let scale = Affine::scale_non_uniform(
            self.size.width / self.svg.size().width() as f64,
            self.size.height / self.svg.size().height() as f64,
        );
        let svg_fragment = vello_svg::render_tree(&self.svg);
        scene.append(&svg_fragment, Some(scale))
    }

    fn debug_name(&self) -> &str {
        "svg"
    }
    fn layout(&mut self, _: Size, _font_context: &mut FontContext) -> Size {
        self.size
    }

    fn children(&self) -> Vec<&super::WidgetData<State>> {
        vec![]
    }

    fn children_mut(&mut self) -> Vec<&mut super::WidgetData<State>> {
        vec![]
    }

    fn event(
        &mut self,
        _event_context: &mut event::EventContext,
        _event: event::WidgetEvent,
        _state: &mut State,
    ) -> event::Status {
        event::Status::Captured
    }

    fn set_hover(&mut self, _hover: bool) -> event::Status {
        event::Status::Ignored
    }
}
