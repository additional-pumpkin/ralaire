use parley::FontContext;
use ralaire_core::{
    event, Affine, AppMessage, AsAny, BlendMode, Point, Rect, RenderCx, RoundedRect,
    RoundedRectRadii, Size, WidgetId, WidgetIdPath,
};
pub trait Widget<Message>: core::fmt::Debug + AsAny
where
    Message: Clone + core::fmt::Debug + 'static,
{
    /// Used by the library to render child widgets and calls draw
    fn render(&self, render_cx: &mut RenderCx) {
        self.draw(render_cx);
        for child in self.children().iter() {
            let bounds = Rect::from_origin_size(child.position, child.size)
                .to_rounded_rect(child.widget.bounds_radii());
            render_cx.push_layer(BlendMode::default(), Affine::default(), bounds);
            child.widget.render(render_cx);
            render_cx.pop_layer();
        }
    }

    fn send_event(
        &mut self,
        event: event::WidgetEvent,
        event_cx: &mut event::EventCx<AppMessage<Message>>,
        mut recipient: WidgetIdPath,
    ) {
        let parent_id = recipient.remove(0);
        if self.event(event.clone(), event_cx) == event::Status::Ignored {
            if let Some(&first) = recipient.first() {
                let child = self.children_mut().into_iter().find(|wd| wd.id == first);
                if let Some(child) = child {
                    let widget_event = event::widget_event(event, child.position);
                    child.widget.send_event(widget_event, event_cx, recipient);
                } else {
                    panic!("Stale widget! Parent has id {:?}", parent_id)
                }
            }
        }
    }

    fn send_hover(&mut self, hover: bool, mut recipient: WidgetIdPath) {
        let parent_id = recipient.remove(0);
        // TODO: Maybe we should send all of them hover status?
        if self.set_hover(hover) == event::Status::Ignored {
            if let Some(&first) = recipient.first() {
                let child = self.children_mut().into_iter().find(|wd| wd.id == first);
                if let Some(child) = child {
                    return child.widget.send_hover(hover, recipient);
                } else {
                    panic!("Stale widget! Parent has id {:?}", parent_id)
                }
            }
        }
    }

    fn bounds_tree(
        &self,
        id_path: WidgetIdPath,
        position: Point,
    ) -> Vec<(WidgetIdPath, RoundedRect)> {
        let mut v = vec![];
        for child in self.children() {
            let mut child_id_path = id_path.clone();
            child_id_path.push(child.id);
            v.push((
                child_id_path.clone(),
                Rect::from_origin_size(position + child.position.to_vec2(), child.size)
                    .to_rounded_rect(child.widget.bounds_radii()),
            ));
            v.extend_from_slice(
                &child
                    .widget
                    .bounds_tree(child_id_path.clone(), position + child.position.to_vec2()),
            )
        }
        v
    }

    fn size_hint(&self) -> WidgetSize;
    // TODO: Consider removing this and allowing any arbitrary shape,
    // alternatively only allow Rects and reconsider hover/focus impl
    fn bounds_radii(&self) -> RoundedRectRadii {
        0.0.into()
    }
    fn layout(&mut self, constraints: Constraints, font_cx: &mut FontContext);
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
    fn children(&self) -> Vec<&WidgetData<Message>>;
    fn children_mut(&mut self) -> Vec<&mut WidgetData<Message>>;
}
#[derive(Debug)]
pub struct WidgetData<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    pub(crate) id: WidgetId,
    pub(crate) position: Point,
    pub(crate) size: Size,
    pub(crate) change_flags: ChangeFlags,
    pub(crate) widget: Box<dyn Widget<Message>>,
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

// #[derive(Debug)]
// pub struct WidgetCx<Message>
// where
//     Message: Clone + core::fmt::Debug + 'static,
// {
//     widget_tree: HashMap<WidgetId, WidgetData<Message>>,
// }

// impl<Message> WidgetCx<Message>
// where
//     Message: Clone + core::fmt::Debug + 'static,
// {
//     pub fn new() -> Self {
//         Self {
//             widget_tree: HashMap::new(),
//         }
//     }
//     pub fn widget(&mut self, widget_id: WidgetId) -> &mut WidgetData<Message> {
//         self.widget_tree.get_mut(&widget_id).unwrap()
//     }
//     pub fn add_widget(&mut self, widget_data: WidgetData<Message>) {
//         self.widget_tree.insert(widget_data.id, widget_data);
//     }

//     pub fn remove_widget(&mut self, widget_id: WidgetId) {
//         fn children_ids<Message>(
//             widget_tree: &HashMap<WidgetId, WidgetData<Message>>,
//             widget_id: WidgetId,
//         ) -> Vec<WidgetId>
//         where
//             Message: Clone + core::fmt::Debug + 'static,
//         {
//             let mut ids = vec![];
//             for child in widget_tree.get(&widget_id).unwrap().widget.children() {
//                 ids.push(child);
//                 ids.extend_from_slice(children_ids(widget_tree, widget_id).as_slice());
//             }
//             ids
//         }
//         let children = children_ids(&self.widget_tree, widget_id);
//         self.widget_tree.remove(&widget_id);
//         for child in children {
//             self.widget_tree.remove(&child);
//         }
//     }
//     pub fn size(&self, widget_id: WidgetId) -> Size {
//         self.widget_tree.get(&widget_id).unwrap().size.clone()
//     }
//     pub fn size_mut(&mut self, widget_id: WidgetId) -> &mut Size {
//         &mut self.widget_tree.get_mut(&widget_id).unwrap().size
//     }
//     pub fn position(&self, widget_id: WidgetId) -> Point {
//         self.widget_tree.get(&widget_id).unwrap().position.clone()
//     }
//     pub fn position_mut(&mut self, widget_id: WidgetId) -> &mut Point {
//         &mut self.widget_tree.get_mut(&widget_id).unwrap().position
//     }

//     pub fn size_hint(&self, widget_id: WidgetId) -> WidgetSize {
//         self.widget_tree.get(&widget_id).unwrap().widget.size_hint()
//     }
//     pub fn bounds_radii(&self, widget_id: WidgetId) -> RoundedRectRadii {
//         self.widget_tree
//             .get(&widget_id)
//             .unwrap()
//             .widget
//             .bounds_radii()
//     }
//     pub fn event(
//         &mut self,
//         widget_id: WidgetId,
//         event: event::WidgetEvent,
//         event_cx: &mut event::EventCx<AppMessage<Message>>,
//     ) -> event::Status {
//         self.widget_tree
//             .get_mut(&widget_id)
//             .unwrap()
//             .widget
//             .event(event, event_cx)
//     }
//     pub fn set_hover(&mut self, widget_id: WidgetId, hover: bool) -> event::Status {
//         self.widget_tree
//             .get_mut(&widget_id)
//             .unwrap()
//             .widget
//             .set_hover(hover)
//     }
//     pub fn layout(
//         &mut self,
//         widget_id: WidgetId,
//         constraints: Constraints,
//         font_cx: &mut FontContext,
//     ) {
//         let mut widget_data = self.widget_tree.remove(&widget_id).unwrap();

//         widget_data.widget.layout(self, constraints, font_cx);
//         self.widget_tree.insert(widget_id, widget_data);
//     }
//     pub fn draw(&self, widget_id: WidgetId, render_cx: &mut RenderCx) {
//         self.widget_tree
//             .get(&widget_id)
//             .unwrap()
//             .widget
//             .draw(render_cx)
//     }
//     pub fn overlay(&self, widget_id: WidgetId, render_cx: &mut RenderCx) {
//         self.widget_tree
//             .get(&widget_id)
//             .unwrap()
//             .widget
//             .overlay(render_cx)
//     }
//     pub fn children(&self, widget_id: WidgetId) -> Vec<&WidgetData<Message>> {
//         self.widget_tree
//             .get(&widget_id)
//             .unwrap()
//             .widget
//             .children()
//             .iter()
//             .map(|widget_id| self.widget_tree.get(widget_id).unwrap())
//             .collect()
//     }
// }
