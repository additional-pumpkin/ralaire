use crate::renderer::{PaintCx, RenderCommand, RenderCx};
use crate::{event, AsAny, WidgetId, WidgetIdPath};
use parley::FontContext;
use peniko::kurbo::{Point, Rect, Size};
pub trait Widget<Message>: AsAny
where
    Message: Clone + core::fmt::Debug + 'static,
{
    fn layout(&mut self, constraints: Constraints, font_cx: &mut FontContext) -> Size;
    fn event(
        &mut self,
        _event: event::WidgetEvent,
        _event_cx: &mut event::EventCx<Message>,
    ) -> event::Status;
    fn set_hover(&mut self, _hover: bool) -> event::Status;
    fn paint(&self, paint_cx: &mut PaintCx);
    fn children(&self) -> Vec<&WidgetData<Message>>;
    fn children_mut(&mut self) -> Vec<&mut WidgetData<Message>>;
    fn debug_name(&self) -> &str;
    /// Used by the library to render child widgets and calls draw
    fn render(&self, render_cx: &mut RenderCx) {
        let mut paint_cx = PaintCx::new();
        self.paint(&mut paint_cx);
        let mut pushed_layers = 0;
        for command in &paint_cx.command_stack {
            match command {
                RenderCommand::PushLayer {
                    blend: _,
                    transform: _,
                    clip: _,
                } => pushed_layers += 1,
                RenderCommand::PopLayer => pushed_layers -= 1,
                _ => {}
            }
        }
        assert!(pushed_layers >= 0);
        render_cx.command_stack.extend(paint_cx.command_stack);
        for child in self.children().iter() {
            let bounds = Rect::from_origin_size(child.position, child.size);
            render_cx.push_widget(child.id, bounds);
            child.widget.render(render_cx);
            render_cx.pop_widget();
        }
        for _ in 0..pushed_layers {
            render_cx.command_stack.push(RenderCommand::PopLayer)
        }
    }
    // TODO: Events should be passed starting at the end of the path
    fn bounds_tree(&self, id_path: WidgetIdPath, position: Point) -> Vec<(WidgetIdPath, Rect)> {
        let mut v = vec![];
        for child in self.children() {
            let mut child_id_path = id_path.clone();
            child_id_path.push(child.id);
            v.push((
                child_id_path.clone(),
                Rect::from_origin_size(position + child.position.to_vec2(), child.size),
            ));
            v.extend_from_slice(
                &child
                    .widget
                    .bounds_tree(child_id_path.clone(), position + child.position.to_vec2()),
            )
        }
        v
    }
}
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
impl<Message> core::fmt::Debug for WidgetData<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let widget_name = self.widget.debug_name();
        let display_name = format!(
            "{widget_name}<id={}, pos={}, size={}>",
            self.id, self.position, self.size
        );
        let children = self.widget.children();
        if children.is_empty() {
            f.write_str(&display_name)
        } else {
            let mut f_tuple = f.debug_tuple(&display_name);
            for child in children {
                f_tuple.field(&child);
            }
            f_tuple.finish()
        }
    }
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
    pub fn with_id(mut self, id: WidgetId) -> Self {
        self.id = id;
        self
    }
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
