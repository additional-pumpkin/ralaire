use std::collections::HashMap;

use parley::FontContext;
use ralaire_core::{event, AppMessage, AsAny, Point, RenderCx, RoundedRectRadii, Size, WidgetId};
pub trait Widget<Message>: core::fmt::Debug + AsAny
where
    Message: Clone + core::fmt::Debug + 'static,
{
    fn size_hint(&self) -> WidgetSize;
    // TODO: Consider removing this and allowing any arbitrary shape,
    // alternatively only allow Rects and reconsider hover/focus impl
    fn bounds_radii(&self) -> RoundedRectRadii {
        0.0.into()
    }
    fn layout(
        &mut self,
        widget_cx: &mut WidgetCx<Message>,
        constraints: Constraints,
        font_cx: &mut FontContext,
    );
    fn event(
        &mut self,
        _event: event::WidgetEvent,
        _event_cx: &mut event::EventCx<AppMessage<Message>>,
    ) -> event::Status {
        event::Status::Ignored
    }
    fn set_hover(&mut self, _hover: bool) -> event::Status {
        event::Status::Ignored
    }
    fn draw(&self, _render_cx: &mut RenderCx) {}
    fn overlay(&self, _render_cx: &mut RenderCx) {}
    fn children(&self) -> Vec<WidgetId> {
        vec![]
    }
    fn set_children(&mut self, _children: Vec<WidgetId>) {}
}
#[derive(Debug)]
pub struct WidgetData<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    pub id: WidgetId,
    pub position: Point,
    pub size: Size,
    pub change_flags: ChangeFlags,
    pub widget: Box<dyn Widget<Message>>,
}

impl<Message> WidgetData<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    pub fn new(widget: Box<dyn Widget<Message>>) -> Self {
        Self {
            id: WidgetId::unique(),
            position: Point::ZERO,
            size: Size::ZERO,
            change_flags: ChangeFlags::default(),
            widget,
        }
    }
    pub fn with_position(mut self, position: Point) -> Self {
        self.position = position;
        self
    }
    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
    pub fn with_id(mut self, id: WidgetId) -> Self {
        self.id = id;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Length {
    Fixed(f64),
    /// A flexible size with a flex-factor. If the widget has no opinion use flex-factor = 1.
    Flexible(u8),
}

#[derive(Debug, Clone, Copy)]
pub struct WidgetSize {
    pub width: Length,
    pub height: Length,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Constraints {
    pub min_size: Size,
    pub max_size: Size,
}

#[derive(Debug, Default)]
pub struct ChangeFlags {
    pub layout: bool,
    pub draw: bool,
}

#[derive(Debug)]
pub struct WidgetCx<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    widget_tree: HashMap<WidgetId, WidgetData<Message>>,
}

impl<Message> WidgetCx<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    pub fn new() -> Self {
        Self {
            widget_tree: HashMap::new(),
        }
    }
    pub fn widget(&mut self, widget_id: WidgetId) -> &mut WidgetData<Message> {
        self.widget_tree.get_mut(&widget_id).unwrap()
    }
    pub fn add_widget(&mut self, widget_data: WidgetData<Message>) {
        self.widget_tree.insert(widget_data.id, widget_data);
    }

    pub fn remove_widget(&mut self, widget_id: WidgetId) {
        fn children_ids<Message>(
            widget_tree: &HashMap<WidgetId, WidgetData<Message>>,
            widget_id: WidgetId,
        ) -> Vec<WidgetId>
        where
            Message: Clone + core::fmt::Debug + 'static,
        {
            let mut ids = vec![];
            for child in widget_tree.get(&widget_id).unwrap().widget.children() {
                ids.push(child);
                ids.extend_from_slice(children_ids(widget_tree, widget_id).as_slice());
            }
            ids
        }
        let children = children_ids(&self.widget_tree, widget_id);
        self.widget_tree.remove(&widget_id);
        for child in children {
            self.widget_tree.remove(&child);
        }
    }
    pub fn size(&self, widget_id: WidgetId) -> Size {
        self.widget_tree.get(&widget_id).unwrap().size.clone()
    }
    pub fn size_mut(&mut self, widget_id: WidgetId) -> &mut Size {
        &mut self.widget_tree.get_mut(&widget_id).unwrap().size
    }
    pub fn position(&self, widget_id: WidgetId) -> Point {
        self.widget_tree.get(&widget_id).unwrap().position.clone()
    }
    pub fn position_mut(&mut self, widget_id: WidgetId) -> &mut Point {
        &mut self.widget_tree.get_mut(&widget_id).unwrap().position
    }

    pub fn size_hint(&self, widget_id: WidgetId) -> WidgetSize {
        self.widget_tree.get(&widget_id).unwrap().widget.size_hint()
    }
    pub fn bounds_radii(&self, widget_id: WidgetId) -> RoundedRectRadii {
        self.widget_tree
            .get(&widget_id)
            .unwrap()
            .widget
            .bounds_radii()
    }
    pub fn event(
        &mut self,
        widget_id: WidgetId,
        event: event::WidgetEvent,
        event_cx: &mut event::EventCx<AppMessage<Message>>,
    ) -> event::Status {
        self.widget_tree
            .get_mut(&widget_id)
            .unwrap()
            .widget
            .event(event, event_cx)
    }
    pub fn set_hover(&mut self, widget_id: WidgetId, hover: bool) -> event::Status {
        self.widget_tree
            .get_mut(&widget_id)
            .unwrap()
            .widget
            .set_hover(hover)
    }
    pub fn layout(
        &mut self,
        widget_id: WidgetId,
        constraints: Constraints,
        font_cx: &mut FontContext,
    ) {
        let mut widget_data = self.widget_tree.remove(&widget_id).unwrap();

        widget_data.widget.layout(self, constraints, font_cx);
        self.widget_tree.insert(widget_id, widget_data);
    }
    pub fn draw(&self, widget_id: WidgetId, render_cx: &mut RenderCx) {
        self.widget_tree
            .get(&widget_id)
            .unwrap()
            .widget
            .draw(render_cx)
    }
    pub fn overlay(&self, widget_id: WidgetId, render_cx: &mut RenderCx) {
        self.widget_tree
            .get(&widget_id)
            .unwrap()
            .widget
            .overlay(render_cx)
    }
    pub fn children(&self, widget_id: WidgetId) -> Vec<&WidgetData<Message>> {
        self.widget_tree
            .get(&widget_id)
            .unwrap()
            .widget
            .children()
            .iter()
            .map(|widget_id| self.widget_tree.get(widget_id).unwrap())
            .collect()
    }
}
