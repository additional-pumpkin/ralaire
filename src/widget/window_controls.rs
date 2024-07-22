use super::{window_button::WindowButtonWidget, Container, Svg, WidgetData};
use crate::widget::Widget;
use crate::{alignment, event, InternalMessage};
use parley::FontContext;
use vello_svg::usvg;
use vello::peniko::kurbo::{Point, Size};

const WINDOW_CONTROLS_WIDTH: f64 = 100.;
const WINDOW_CONTROLS_HEIGHT: f64 = 46.;

#[derive(Debug)]
pub struct WindowControls<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    // TODO: Support 'restore' icon on maximise button
    // Widgets need some sort of reactivity on their own.
    // Currently widgets react to *window* events and emit
    // InternaMessages which the AppWindow can react to.
    // In this case we could send a *custom* event notifying
    // that the window has been maximised to this widget.
    buttons: Vec<WidgetData<Message>>,
}

impl<Message> WindowControls<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    pub fn new() -> Self {
        let close_svg = usvg::Tree::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/assets/icons/window-close-symbolic.svg")), &usvg::Options::default()).unwrap();
        let maximize_svg = usvg::Tree::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/assets/icons/window-maximize-symbolic.svg")), &usvg::Options::default()).unwrap();
        let minimize_svg = usvg::Tree::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/assets/icons/window-minimize-symbolic.svg")), &usvg::Options::default()).unwrap();
        let close_icon =
            WidgetData::new(Box::new(Svg::new(close_svg)));
        let close_button = WidgetData::new(Box::new(WindowButtonWidget::new(
            close_icon,
            InternalMessage::CloseWindow,
        )));
        let close_button = WidgetData::new(Box::new(Container::new(
            close_button,
            alignment::Horizontal::Center,
            alignment::Vertical::Center,
            0.0.into(),
        )));
        let maximise_icon = WidgetData::new(Box::new(Svg::new(
            maximize_svg
        )));
        let maximise_button = WidgetData::new(Box::new(WindowButtonWidget::new(
            maximise_icon,
            InternalMessage::MaximiseWindow,
        )));
        let maximise_button = WidgetData::new(Box::new(Container::new(
            maximise_button,
            alignment::Horizontal::Center,
            alignment::Vertical::Center,
            0.0.into(),
        )));
        let minimise_icon = WidgetData::new(Box::new(Svg::new(
            minimize_svg
        )));
        let minimise_button = WidgetData::new(Box::new(WindowButtonWidget::new(
            minimise_icon,
            InternalMessage::MinimiseWindow,
        )));
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

impl<Message> Widget<Message> for WindowControls<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn debug_name(&self) -> &str {
        "window_controls"
    }
    fn layout(&mut self, _size_hint: Size, font_cx: &mut FontContext) -> Size {
        let button_size_hint = Size::new(WINDOW_CONTROLS_WIDTH / 3., WINDOW_CONTROLS_HEIGHT);
        let number_of_buttons = self.buttons.len();
        for (idx, button) in self.buttons.iter_mut().enumerate() {
            button.size = button.layout(button_size_hint, font_cx);
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
    fn children(&self) -> Vec<&super::WidgetData<Message>> {
        self.buttons.iter().collect()
    }
    fn children_mut(&mut self) -> Vec<&mut super::WidgetData<Message>> {
        self.buttons.iter_mut().collect()
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
