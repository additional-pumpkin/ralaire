use crate::widget::alignment;
use crate::widget::{Container, Svg, Widget, WidgetData, WidgetMarker, WindowButtonWidget};
use crate::{event, InternalMessage};
use parley::FontContext;
use vello::peniko::kurbo::{Point, Size};
use vello_svg::usvg;

const WINDOW_CONTROLS_WIDTH: f64 = 100.;
const WINDOW_CONTROLS_HEIGHT: f64 = 46.;

#[derive(Debug)]
pub struct WindowControls<State: 'static> {
    // TODO: Support 'restore' icon on maximise button
    // Widgets need some sort of reactivity on their own.
    // Currently widgets react to *window* events and emit
    // InternaMessages which the AppWindow can react to.
    // In this case we could send a *custom* event notifying
    // that the window has been maximised to this widget.
    buttons: Vec<WidgetData<State>>,
}

impl<State: 'static> WindowControls<State> {
    pub fn new() -> Self {
        tracing::warn!("Creating window control buttons!!");
        let close_svg = usvg::Tree::from_str(
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/icons/window-close-symbolic.svg"
            )),
            &usvg::Options::default(),
        )
        .unwrap();
        let maximize_svg = usvg::Tree::from_str(
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/icons/window-maximize-symbolic.svg"
            )),
            &usvg::Options::default(),
        )
        .unwrap();
        let minimize_svg = usvg::Tree::from_str(
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/icons/window-minimize-symbolic.svg"
            )),
            &usvg::Options::default(),
        )
        .unwrap();
        let close_icon = Svg::new(close_svg);
        let close_button = WindowButtonWidget::new(close_icon, InternalMessage::CloseWindow);
        let close_button = WidgetData::new(Box::new(Container::new(
            close_button,
            alignment::Horizontal::Center,
            alignment::Vertical::Center,
            0.0.into(),
        )));
        let maximise_icon = Svg::new(maximize_svg);
        let maximise_button =
            WindowButtonWidget::new(maximise_icon, InternalMessage::MaximiseWindow);
        let maximise_button = WidgetData::new(Box::new(Container::new(
            maximise_button,
            alignment::Horizontal::Center,
            alignment::Vertical::Center,
            0.0.into(),
        )));
        let minimise_icon = Svg::new(minimize_svg);
        let minimise_button =
            WindowButtonWidget::new(minimise_icon, InternalMessage::MinimiseWindow);
        let minimise_button = WidgetData::new(Box::new(Container::new(
            minimise_button,
            alignment::Horizontal::Center,
            alignment::Vertical::Center,
            0.0.into(),
        )));
        Self {
            buttons: vec![close_button, maximise_button, minimise_button],
        }
    }
}

impl<State> WidgetMarker for WindowControls<State> {}
impl<State: 'static> Widget<State> for WindowControls<State> {
    fn debug_name(&self) -> &str {
        "window_controls"
    }
    fn layout(&mut self, _: Size, font_context: &mut FontContext) -> Size {
        let button_size_hint = Size::new(WINDOW_CONTROLS_WIDTH / 3., WINDOW_CONTROLS_HEIGHT);
        let number_of_buttons = self.buttons.len();
        for (idx, button) in self.buttons.iter_mut().enumerate() {
            button.size = button.layout(button_size_hint, font_context);
            button.position = Point::new(
                WINDOW_CONTROLS_WIDTH * (number_of_buttons - 1 - idx) as f64
                    / number_of_buttons as f64,
                0.,
            );
        }
        Size::new(WINDOW_CONTROLS_WIDTH, WINDOW_CONTROLS_HEIGHT)
    }
    fn paint(&mut self, scene: &mut vello::Scene) {
        for child in self.children_mut() {
            child.paint(scene);
        }
    }
    fn children(&self) -> Vec<&super::WidgetData<State>> {
        self.buttons.iter().collect()
    }
    fn children_mut(&mut self) -> Vec<&mut super::WidgetData<State>> {
        self.buttons.iter_mut().collect()
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
