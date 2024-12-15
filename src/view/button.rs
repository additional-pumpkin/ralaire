use std::marker::PhantomData;

use crate::view::{View, ViewMarker};
use crate::widget::{self, Widget};
use vello::peniko::kurbo::{RoundedRectRadii, Size};
use vello::peniko::Color;

pub fn button<State, Child>(child: Child) -> Button<State, Child> {
    Button::new(child)
}

pub struct Button<State, Child> {
    size: Size,
    radii: RoundedRectRadii,
    color: Color,
    on_press: Option<fn(&mut State)>,
    child: Child,
    phantom_data: PhantomData<State>,
}

impl<State, Child> Button<State, Child> {
    pub fn new(child: Child) -> Self {
        Self {
            size: Size::new(60. * 1.5, 23. * 1.5),
            radii: 10.0.into(),
            color: Color::LIGHT_GRAY,
            on_press: None,
            child,
            phantom_data: PhantomData,
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

impl<State, Child> ViewMarker for Button<State, Child> {}

impl<State: 'static, Child: View<State>> View<State> for Button<State, Child>
where
    Child::Element: Widget<State>,
{
    type Element = widget::Button<State>;
    fn build(&self) -> Self::Element {
        let child = self.child.build();
        widget::Button::new(
            child,
            self.size,
            self.radii,
            self.color,
            self.on_press
                .map(|f| Box::new(f.clone()) as Box<dyn Fn(&mut State) + Send + Sync + 'static>),
        )
    }

    fn rebuild(&self, old: &Self, element: &mut Self::Element) {
        if self.size != old.size || self.color != old.color || self.radii != old.radii {
            if element.size != self.size {
                element.size = self.size;
            }
            element.color = self.color;
            element.radii = self.radii;
            element.on_press = self
                .on_press
                .map(|f| Box::new(f.clone()) as Box<dyn Fn(&mut State) + Send + Sync + 'static>);
        }
        // there is only one child...
        for child in element.children_mut() {
            self.child.rebuild(
                &old.child,
                (*child.inner)
                    .as_any_mut()
                    .downcast_mut::<Child::Element>()
                    .unwrap(),
            )
        }
    }
    fn teardown(&self, element: &mut Self::Element) {
        for child in element.children_mut() {
            self.child.teardown(
                (*child.inner)
                    .as_any_mut()
                    .downcast_mut::<Child::Element>()
                    .unwrap(),
            )
        }
    }
}
