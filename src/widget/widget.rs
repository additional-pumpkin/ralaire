use crate::app::AppMessage;
use parley::FontContext;
use ralaire_core::{
    event, Affine, BlendMode, Id, IdPath, Point, Rect, RenderCx, RoundedRect, RoundedRectRadii,
    Size,
};
pub trait Widget<Message>: std::fmt::Debug
where
    Message: Clone + std::fmt::Debug,
{
    fn size_hint(&self) -> WidgetSize;
    fn bounds_radii(&self) -> RoundedRectRadii {
        0.0.into()
    }
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
    fn children(&self) -> Vec<&WidgetData<Message>>;
    fn children_mut(&mut self) -> Vec<&mut WidgetData<Message>>;
    fn layout(&mut self, constraints: Constraints, font_cx: &mut FontContext);
    /// Used by the user to draw the widget (uses local coordinates)
    fn draw(&self, _render_cx: &mut RenderCx) {}
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
    // The recipient might be stale, this function returns the number of stale Ids from the end (so they can be removed)
    fn send_event(
        &mut self,
        event: event::WidgetEvent,
        event_cx: &mut event::EventCx<AppMessage<Message>>,
        mut recipient: IdPath,
    ) -> usize {
        recipient.remove(0);
        if self.event(event.clone(), event_cx) == event::Status::Ignored {
            if let Some(&first) = recipient.first() {
                let child = self.children_mut().into_iter().find(|wd| wd.id == first);
                if let Some(child) = child {
                    let widget_event = event::widget_event(event, child.position);
                    return child.widget.send_event(widget_event, event_cx, recipient);
                } else {
                    return recipient.len();
                }
            }
        }
        0
    }
    fn bounds_tree(&self, id_path: IdPath, position: Point) -> Vec<(IdPath, RoundedRect)> {
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
    // The recipient might be stale, this function returns the number of stale Ids from the end (so they can be removed)
    fn send_hover(&mut self, hover: bool, mut recipient: IdPath) -> usize {
        recipient.remove(0);
        if self.set_hover(hover) == event::Status::Ignored {
            if let Some(&first) = recipient.first() {
                let child = self.children_mut().into_iter().find(|wd| wd.id == first);
                if let Some(child) = child {
                    return child.widget.send_hover(hover, recipient);
                } else {
                    return recipient.len();
                }
            }
        }
        0
    }
}
#[derive(Debug)]
pub struct WidgetData<Message> {
    pub id: Id,
    pub position: Point,
    pub size: Size,
    pub widget: Box<dyn Widget<Message>>,
}

impl<Message> WidgetData<Message>
where
    Message: Clone + std::fmt::Debug,
{
    pub fn new(widget: impl Widget<Message> + 'static) -> Self {
        WidgetData {
            id: Id::unique(),
            position: Point::ZERO,
            size: Size::ZERO,
            widget: Box::new(widget),
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
    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }
}

#[derive(Debug, Clone, Copy)]
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
pub struct Constraints {
    pub min_size: Size,
    pub max_size: Size,
}
