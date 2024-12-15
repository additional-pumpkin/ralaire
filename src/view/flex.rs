use crate::view::{ElementSplice, SuperElement, View, ViewMarker, ViewSequence};
use crate::widget::{
    self, CrossAxisAlignment, FlexAxis, FlexChild, FlexDirection, JustifyContent, Widget,
    WidgetData, WidgetMarker,
};

pub fn flex<Seq>(children: Seq) -> Flex<Seq> {
    Flex::new(children)
}

pub struct Flex<Seq> {
    children: Seq,
    flex_direction: FlexDirection,
    cross_axis_alignment: CrossAxisAlignment,
    justify_content: JustifyContent,
}
impl<Seq> Flex<Seq> {
    pub(crate) fn new(children: Seq) -> Flex<Seq> {
        Flex {
            children,
            flex_direction: FlexDirection::Column,
            cross_axis_alignment: CrossAxisAlignment::Start,
            justify_content: JustifyContent::Start,
        }
    }
    pub fn direction(mut self, flex_direction: FlexDirection) -> Self {
        self.flex_direction = flex_direction;
        self
    }
    pub fn cross_axis_alignment(mut self, cross_axis_alignment: CrossAxisAlignment) -> Self {
        self.cross_axis_alignment = cross_axis_alignment;
        self
    }
    pub fn justify_content(mut self, justify_content: JustifyContent) -> Self {
        self.justify_content = justify_content;
        self
    }
}
impl<Seq> ViewMarker for Flex<Seq> {}

impl<State: 'static, Seq> View<State> for Flex<Seq>
where
    Seq: ViewSequence<State, FlexChild<State>> + 'static,
{
    type Element = widget::Flex<State>;
    fn build(&self) -> Self::Element {
        let mut e = vec![];
        self.children.seq_build(&mut e);
        widget::Flex::new(
            e,
            self.flex_direction,
            self.cross_axis_alignment,
            self.justify_content,
        )
    }

    fn rebuild(&self, old: &Self, element: &mut Self::Element) {
        if self.flex_direction != old.flex_direction {
            let (flex_axis, direction_flipped) = match self.flex_direction {
                FlexDirection::Column => (FlexAxis::Vertical, false),
                FlexDirection::Row => (FlexAxis::Horizontal, false),
                FlexDirection::ColumnReversed => (FlexAxis::Vertical, true),
                FlexDirection::RowReversed => (FlexAxis::Horizontal, true),
            };
            element.main_axis = flex_axis;
            element.direction_flipped = direction_flipped;
        }
        if self.cross_axis_alignment != old.cross_axis_alignment {}

        let mut splice = FlexSplice::new(element);
        self.children.seq_rebuild(&old.children, &mut splice);
    }

    fn teardown(&self, element: &mut Self::Element) {
        let mut splice = FlexSplice::new(element);
        self.children.seq_teardown(&mut splice);
    }
}

struct FlexSplice<'a, State> {
    idx: usize,
    flex: &'a mut widget::Flex<State>,
}

impl<'a, State> FlexSplice<'a, State> {
    fn new(flex: &'a mut widget::Flex<State>) -> Self {
        Self { idx: 0, flex }
    }
}

impl<'a, State> ElementSplice<FlexChild<State>> for FlexSplice<'a, State> {
    fn with_scratch<R>(&mut self, f: impl FnOnce(&mut Vec<FlexChild<State>>) -> R) -> R {
        let mut scratch = vec![];
        let ret = f(&mut scratch);
        for element in scratch.drain(..) {
            self.flex.insert_child(self.idx, element);
            self.idx += 1;
        }
        ret
    }

    fn insert(&mut self, element: FlexChild<State>) {
        self.flex.insert_child(self.idx, element);
        self.idx += 1;
    }

    fn mutate<R>(&mut self, f: impl FnOnce(&mut FlexChild<State>) -> R) -> R {
        let child = self.flex.mutate_child(self.idx);
        self.idx += 1;
        f(child)
    }

    fn skip(&mut self, n: usize) {
        self.idx += n;
    }

    fn remove<R>(&mut self, f: impl FnOnce(&mut FlexChild<State>) -> R) -> R {
        let child = self.flex.mutate_child(self.idx);
        let ret = f(child);
        self.flex.remove_child(self.idx);
        ret
    }
}

impl<State: 'static> SuperElement<FlexChild<State>> for FlexChild<State> {
    fn upcast(child: FlexChild<State>) -> Self {
        child
    }

    fn with_downcast_val<R>(
        &mut self,
        f: impl FnOnce(&mut FlexChild<State>) -> R,
    ) -> (&mut Self, R) {
        let ret = f(self);
        (self, ret)
    }
}

impl<State: 'static, W> SuperElement<W> for FlexChild<State>
where
    W: Widget<State> + WidgetMarker,
{
    fn upcast(child: W) -> Self {
        FlexChild {
            widget: WidgetData::new(Box::new(child)),
            flex_factor: None,
            cross_axis_alignment: None,
        }
    }

    fn with_downcast_val<R>(&mut self, f: impl FnOnce(&mut W) -> R) -> (&mut Self, R) {
        let ret = f(&mut self.widget.inner.as_any_mut().downcast_mut::<W>().unwrap());
        (self, ret)
    }
}

// TODO: implement a more general system that can also take other attributes like margins
pub trait FlexExt<State: 'static>: View<State> {
    fn flex(self, flex_factor: f64) -> FlexItem<Self>
    where
        Self: Sized,
    {
        FlexItem {
            view: self,
            flex_factor: Some(flex_factor),
            cross_axis_alignment: None,
        }
    }
    fn align_self(self, cross_axis_alignment: CrossAxisAlignment) -> FlexItem<Self>
    where
        Self: Sized,
    {
        FlexItem {
            view: self,
            flex_factor: None,
            cross_axis_alignment: Some(cross_axis_alignment),
        }
    }
}

impl<State: 'static, V: View<State>> FlexExt<State> for V {}

pub struct FlexItem<V> {
    view: V,
    flex_factor: Option<f64>,
    cross_axis_alignment: Option<CrossAxisAlignment>,
}

impl<V> ViewMarker for FlexItem<V> {}
impl<State, V> View<State> for FlexItem<V>
where
    State: 'static,
    V: View<State>,
    V::Element: Widget<State>,
{
    type Element = FlexChild<State>;

    fn build(&self) -> Self::Element {
        let widget = self.view.build();
        FlexChild {
            widget: WidgetData::new(Box::new(widget)),
            flex_factor: self.flex_factor,
            cross_axis_alignment: self.cross_axis_alignment,
        }
    }

    fn rebuild(&self, old: &Self, element: &mut Self::Element) {
        {
            if self.flex_factor != old.flex_factor {
                element.flex_factor = self.flex_factor;
            }
            if self.cross_axis_alignment != old.cross_axis_alignment {
                element.cross_axis_alignment = self.cross_axis_alignment;
            }
            self.view.rebuild(
                &old.view,
                (*element.widget.inner)
                    .as_any_mut()
                    .downcast_mut::<V::Element>()
                    .unwrap(),
            );
        }
    }

    fn teardown(&self, element: &mut Self::Element) {
        self.view.teardown(
            (*element.widget.inner)
                .as_any_mut()
                .downcast_mut::<V::Element>()
                .unwrap(),
        );
    }
}
