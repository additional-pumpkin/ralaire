use crate::event::{self, mouse::MouseButton, WidgetEvent};
use crate::widget::{Widget, WidgetData, WidgetMarker, WindowControls};
use parley::FontContext;
use vello::kurbo::{Point, Size};
const WINDOW_CONTROLS_WIDTH: f64 = 100.;
const HEADER_HEIGHT: f64 = 46.;

/// like bar but includes window controls (for example minimise, maximise, close)
pub struct Header<State> {
    width: f64,
    pub left: WidgetData<State>,
    pub middle: WidgetData<State>,
    pub right: WidgetData<State>,
    pub window_controls: WidgetData<State>,
}

#[allow(dead_code)]
impl<State: 'static> Header<State> {
    pub fn new<LW: Widget<State>, MW: Widget<State>, RW: Widget<State>>(
        left: LW,
        middle: MW,
        right: RW,
        window_controls: WindowControls<State>,
    ) -> Self {
        Self {
            width: 0.,
            left: WidgetData::new(Box::new(left)),
            middle: WidgetData::new(Box::new(middle)),
            right: WidgetData::new(Box::new(right)),
            window_controls: WidgetData::new(Box::new(window_controls)),
        }
    }
}

impl<State> WidgetMarker for Header<State> {}
impl<State: 'static> Widget<State> for Header<State> {
    fn debug_name(&self) -> &str {
        "header"
    }
    fn paint(&mut self, scene: &mut vello::Scene) {
        for child in self.children_mut() {
            child.paint(scene);
        }
    }

    fn children(&self) -> Vec<&WidgetData<State>> {
        vec![&self.left, &self.middle, &self.right, &self.window_controls]
    }

    fn children_mut(&mut self) -> Vec<&mut WidgetData<State>> {
        vec![
            &mut self.left,
            &mut self.middle,
            &mut self.right,
            &mut self.window_controls,
        ]
    }

    fn layout(&mut self, suggested_size: Size, font_context: &mut FontContext) -> Size {
        if !suggested_size.is_finite() {
            panic!("FIXME: size is infinite");
        }
        self.width = suggested_size.width;
        let side_size_hint = Size {
            width: f64::INFINITY,
            height: HEADER_HEIGHT,
        };
        let left_width = self.left.layout(side_size_hint, font_context).width;
        let right_width = self.right.layout(side_size_hint, font_context).width;
        let max_width = f64::max(left_width, right_width + WINDOW_CONTROLS_WIDTH);
        let middle_width = suggested_size.width - max_width * 2.;
        self.left.size = Size::new(max_width, HEADER_HEIGHT);
        self.left.position = Point::new(0., 0.);
        self.right.size = Size::new(max_width, HEADER_HEIGHT);
        self.right.position = Point::new(max_width + middle_width, 0.);
        let middle_size_suggestion = Size {
            width: middle_width,
            height: HEADER_HEIGHT,
        };
        // TODO: handle all sizes within constraints
        self.middle.layout(middle_size_suggestion, font_context);
        self.middle.size = Size::new(middle_width, HEADER_HEIGHT);
        self.middle.position = Point::new(max_width, 0.);
        self.window_controls.inner.layout(
            Size::new(WINDOW_CONTROLS_WIDTH, HEADER_HEIGHT),
            font_context,
        );
        self.window_controls.position = Point::new(self.width - WINDOW_CONTROLS_WIDTH, 0.);
        self.window_controls.size = Size::new(WINDOW_CONTROLS_WIDTH, HEADER_HEIGHT);

        suggested_size
    }

    fn event(
        &mut self,
        event_context: &mut event::EventContext,
        event: event::WidgetEvent,
        _state: &mut State,
    ) -> event::Status {
        if let WidgetEvent::Mouse(event::mouse::Event::Press {
            position: _,
            button: MouseButton::Left,
        }) = event.clone()
        {
            let _ = event_context.winit_window.drag_window();
            return event::Status::Captured;
        }
        event::Status::Ignored
    }

    fn set_hover(&mut self, _hover: bool) -> event::Status {
        event::Status::Ignored
    }
}
