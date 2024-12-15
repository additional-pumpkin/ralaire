use crate::widget::{Widget, WidgetData, WidgetMarker};
use crate::{event, Padding};
use parley::FontContext;
use vello::kurbo::{Point, Size};

pub mod alignment {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Horizontal {
        Left,
        Center,
        Right,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Vertical {
        Top,
        Center,
        Bottom,
    }
}
pub struct Container<State> {
    size: Size,
    pub(crate) h_alignment: alignment::Horizontal,
    pub(crate) v_alignment: alignment::Vertical,
    pub(crate) padding: Padding,
    child: WidgetData<State>,
}

impl<State: 'static> Container<State> {
    pub fn new(
        child: impl Widget<State>,
        h_alignment: alignment::Horizontal,
        v_alignment: alignment::Vertical,
        padding: Padding,
    ) -> Self {
        Container {
            size: Size::ZERO,
            h_alignment,
            v_alignment,
            padding,
            child: WidgetData::new(Box::new(child)),
        }
    }
}

impl<State> WidgetMarker for Container<State> {}
impl<State: 'static> Widget<State> for Container<State> {
    fn paint(&mut self, scene: &mut vello::Scene) {
        self.child.paint(scene);
    }
    fn debug_name(&self) -> &str {
        "container"
    }
    fn children(&self) -> Vec<&WidgetData<State>> {
        vec![&self.child]
    }
    fn children_mut(&mut self) -> Vec<&mut WidgetData<State>> {
        vec![&mut self.child]
    }
    fn layout(&mut self, suggested_size: Size, font_context: &mut FontContext) -> Size {
        if !suggested_size.is_finite() {
            panic!("FIXME: size is infinite");
        }
        self.size = suggested_size;
        self.child.size = self.child.layout(
            Size::new(
                self.size.width - self.padding.horizontal(),
                self.size.height - self.padding.vertical(),
            ),
            font_context,
        );

        let padding = self.padding;
        let x = match self.h_alignment {
            alignment::Horizontal::Left => padding.left,
            alignment::Horizontal::Center => {
                (self.size.width - padding.horizontal() - self.child.size.width) / 2. + padding.left
            }
            alignment::Horizontal::Right => self.size.width - padding.right - self.child.size.width,
        };
        let y = match self.v_alignment {
            alignment::Vertical::Top => padding.top,
            alignment::Vertical::Center => {
                (self.size.height - padding.vertical() - self.child.size.height) / 2. + padding.top
            }
            alignment::Vertical::Bottom => {
                self.size.height - padding.bottom - self.child.size.height
            }
        };
        self.child.position = Point::new(x.max(0.), y.max(0.));
        self.size
    }

    fn event(
        &mut self,
        _event_context: &mut event::EventContext,
        _event: event::WidgetEvent,
        _state: &mut State,
    ) -> event::Status {
        event::Status::Ignored
    }

    fn set_hover(&mut self, _hover: bool) -> event::Status {
        event::Status::Ignored
    }
}
