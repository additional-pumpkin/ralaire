use crate::view::View;
use crate::widget::{self, WidgetData};
use vello::peniko::kurbo::{RoundedRectRadii, Size};
use vello::peniko::Color;

pub fn button<State>(child: impl View<State>) -> Button<State> {
    Button::new(Box::new(child))
}

pub struct Button<State> {
    size: Size,
    radii: RoundedRectRadii,
    color: Color,
    on_press: Option<fn(&mut State)>,
    child: Box<dyn View<State>>,
}

impl<State> Button<State> {
    pub fn new(child: Box<dyn View<State>>) -> Self {
        Self {
            size: Size::new(60. * 1.5, 23. * 1.5),
            radii: 10.0.into(),
            color: Color::LIGHT_GRAY,
            on_press: None,
            child,
        }
    }
    pub fn radius(mut self, radius: f64) -> Self {
        self.radii = radius.into();
        self
    }
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    pub fn on_press(mut self, on_press: fn(&mut State)) -> Self {
        self.on_press = Some(on_press);
        self
    }
}

impl<State: 'static> View<State> for Button<State> {
    fn build_widget(&self) -> WidgetData<State> {
        let child = self.child.build_widget();
        let button: widget::Button<State> = widget::Button::new(
            child,
            self.size,
            self.radii,
            self.color,
            self.on_press
                .map(|f| Box::new(f.clone()) as Box<dyn Fn(&mut State) + Send + Sync + 'static>),
        );
        WidgetData::new(Box::new(button))
    }

    fn change_widget(&self, widget_data: &mut WidgetData<State>) {
        let button = (*widget_data.inner)
            .as_any_mut()
            .downcast_mut::<widget::Button<State>>()
            .unwrap();
        if button.size != self.size {
            button.size = self.size;
            widget_data.change_flags.needs_layout = true;
        }
        button.color = self.color;
        button.radii = self.radii;
        widget_data.change_flags.needs_paint = true;
        button.on_press = self
            .on_press
            .map(|f| Box::new(f.clone()) as Box<dyn Fn(&mut State) + Send + Sync + 'static>);
    }

    fn reconciliate(&self, old: &Box<dyn View<State>>, widget: &mut WidgetData<State>) {
        let old = (**old).as_any().downcast_ref::<Button<State>>().unwrap();
        if self.size != old.size || self.color != old.color || self.radii != old.radii
        // || self.on_press != old.on_press
        {
            self.change_widget(widget)
        }
        // there is only one child...
        for child in widget.inner.children_mut() {
            self.child.reconciliate(&old.child, child)
        }
    }
}
