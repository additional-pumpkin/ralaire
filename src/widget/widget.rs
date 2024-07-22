use crate::{event, AsAny, WidgetId, WidgetIdPath};
use parley::FontContext;
use vello::peniko::kurbo::{Point, Rect, Size};
pub trait Widget<Message>: AsAny
where
    Message: Clone + core::fmt::Debug + 'static,
{
    fn layout(&mut self, suggested_size: Size, font_cx: &mut FontContext) -> Size;
    fn event(
        &mut self,
        _event: event::WidgetEvent,
        _event_cx: &mut event::EventCx<Message>,
    ) -> event::Status;
    fn set_hover(&mut self, _hover: bool) -> event::Status;
    fn paint(&mut self, scene: &mut vello::Scene);
    fn children(&self) -> Vec<&WidgetData<Message>>;
    fn children_mut(&mut self) -> Vec<&mut WidgetData<Message>>;
    fn debug_name(&self) -> &str;
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
                &child.bounds_tree(child_id_path.clone(), position + child.position.to_vec2()),
            )
        }
        v
    }
}
#[derive(Debug, Default)]
pub struct ChangeFlags {
    pub needs_layout: bool,
    pub needs_paint: bool,
}
pub struct WidgetData<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    pub(crate) id: WidgetId,
    pub(crate) position: Point,
    pub(crate) size: Size,
    pub(crate) change_flags: ChangeFlags,
    scene: vello::Scene,
    pub(crate) inner: Box<dyn Widget<Message>>,
}
impl<Message> core::fmt::Debug for WidgetData<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let widget_name = self.inner.debug_name();
        let display_name = format!(
            "{widget_name}<id={}, pos={}, size={}>",
            self.id, self.position, self.size
        );
        let children = self.inner.children();
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
            change_flags: ChangeFlags {
                needs_layout: true,
                needs_paint: true,
            },
            scene: vello::Scene::new(),
            inner: widget,
        }
    }
    pub fn with_id(mut self, id: WidgetId) -> Self {
        self.id = id;
        self
    }
    pub fn layout(&mut self, suggested_size: Size, font_cx: &mut FontContext) -> Size {
        self.change_flags.needs_paint = true;
        self.inner.layout(suggested_size, font_cx)
        // self.size
    }
    pub fn paint(&mut self, scene: &mut vello::Scene) {
        if self.change_flags.needs_paint {
            // self.change_flags.needs_paint = false;
            self.scene.reset();
            self.inner.paint(&mut self.scene);
        }
        let transform = vello::kurbo::Affine::translate(self.position.to_vec2());
        scene.append(&self.scene, Some(transform));
    }
    pub fn bounds_tree(&self, id_path: WidgetIdPath, position: Point) -> Vec<(WidgetIdPath, Rect)> {
        self.inner.bounds_tree(id_path, position)
    }
}
