use super::{window_button::WindowButtonWidget, ContainerWidget, SvgWidget, WidgetData};
use crate::widget::{Constraints, Widget};
use crate::{alignment, event, InternalMessage};
use parley::FontContext;
use vello::peniko::kurbo::{Affine, Point, Size};
use vello::peniko::Color;

const WINDOW_CONTROLS_WIDTH: f64 = 100.;
const WINDOW_CONTROLS_HEIGHT: f64 = 46.;

#[derive(Debug)]
pub struct WindowControlsWidget<Message>
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

impl<Message> WindowControlsWidget<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    pub fn new() -> Self {
        let close_icon = WidgetData::new(Box::new(SvgWidget::new(
            "assets/icons/window-close-symbolic.svg",
        )));
        let close_button = WidgetData::new(Box::new(WindowButtonWidget::new(
            close_icon,
            InternalMessage::CloseWindow,
        )));
        let close_button = WidgetData::new(Box::new(ContainerWidget::new(
            close_button,
            alignment::Horizontal::Center,
            alignment::Vertical::Center,
            0.0.into(),
        )));
        let maximise_icon = WidgetData::new(Box::new(SvgWidget::new(
            "assets/icons/window-maximize-symbolic.svg",
        )));
        let maximise_button = WidgetData::new(Box::new(WindowButtonWidget::new(
            maximise_icon,
            InternalMessage::MaximiseWindow,
        )));
        let maximise_button = WidgetData::new(Box::new(ContainerWidget::new(
            maximise_button,
            alignment::Horizontal::Center,
            alignment::Vertical::Center,
            0.0.into(),
        )));
        let minimise_icon = WidgetData::new(Box::new(SvgWidget::new(
            "assets/icons/window-minimize-symbolic.svg",
        )));
        let minimise_button = WidgetData::new(Box::new(WindowButtonWidget::new(
            minimise_icon,
            InternalMessage::MinimiseWindow,
        )));
        let minimise_button = WidgetData::new(Box::new(ContainerWidget::new(
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

impl<Message> Widget<Message> for WindowControlsWidget<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn debug_name(&self) -> &str {
        "window_controls"
    }
    fn layout(&mut self, _constraints: Constraints, font_cx: &mut FontContext) -> Size {
        let button_constraints = Constraints {
            min_size: Size::new(WINDOW_CONTROLS_WIDTH / 3., WINDOW_CONTROLS_HEIGHT),
            max_size: Size::new(WINDOW_CONTROLS_WIDTH / 3., WINDOW_CONTROLS_HEIGHT),
        };
        let number_of_buttons = self.buttons.len();
        for (idx, button) in self.buttons.iter_mut().enumerate() {
            button.size = button.widget.layout(button_constraints, font_cx);
            button.position = Point::new(
                WINDOW_CONTROLS_WIDTH * (number_of_buttons - 1 - idx) as f64
                    / number_of_buttons as f64,
                0.,
            );
        }
        Size::new(WINDOW_CONTROLS_WIDTH, WINDOW_CONTROLS_HEIGHT)
    }
    fn paint(&self, scene: &mut vello::Scene) {
        for child in self.children() {
            let mut fragment = vello::Scene::new();
            child.widget.paint(&mut fragment);
            let affine = Affine::translate(child.position.to_vec2());
            scene.append(&fragment, Some(affine));
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
