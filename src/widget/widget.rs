use crate::{event, AsAny};
use parley::FontContext;
use vello::{
    kurbo::Affine,
    peniko::kurbo::{Point, Rect, Size},
};
pub trait Widget<State: 'static>: AsAny {
    fn layout(&mut self, suggested_size: Size, font_cx: &mut FontContext) -> Size;
    fn event(
        &mut self,
        event_cx: &mut event::EventCx,
        event: event::WidgetEvent,
        state: &mut State,
    ) -> event::Status;
    fn set_hover(&mut self, _hover: bool) -> event::Status;
    fn paint(&mut self, scene: &mut vello::Scene);
    fn children(&self) -> Vec<&WidgetData<State>>;
    fn children_mut(&mut self) -> Vec<&mut WidgetData<State>>;
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
pub struct WidgetData<State> {
    pub(crate) id: WidgetId,
    pub(crate) position: Point,
    pub(crate) size: Size,
    pub(crate) change_flags: ChangeFlags,
    scene: vello::Scene,
    pub(crate) inner: Box<dyn Widget<State>>,
}
impl<State: 'static> core::fmt::Debug for WidgetData<State> {
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

impl<State: 'static> WidgetData<State> {
    pub fn new(widget: Box<dyn Widget<State>>) -> Self {
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
        let transform = Affine::translate(self.position.to_vec2());
        scene.append(&self.scene, Some(transform));
    }
    pub fn bounds_tree(&self, id_path: WidgetIdPath, position: Point) -> Vec<(WidgetIdPath, Rect)> {
        self.inner.bounds_tree(id_path, position)
    }
}

use core::{
    num::NonZeroU64,
    sync::atomic::{AtomicU64, Ordering},
};
extern crate alloc;
use alloc::vec::Vec;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub struct WidgetId(NonZeroU64);

pub type WidgetIdPath = Vec<WidgetId>;

impl WidgetId {
    pub fn unique() -> WidgetId {
        static WIDGET_ID_COUNTER: AtomicU64 = AtomicU64::new(1);
        WidgetId(NonZeroU64::new(WIDGET_ID_COUNTER.fetch_add(1, Ordering::Relaxed)).unwrap())
    }

    pub fn to_raw(self) -> u64 {
        self.0.into()
    }

    pub fn to_nonzero_raw(self) -> NonZeroU64 {
        self.0
    }
}

impl core::fmt::Display for WidgetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.0))
    }
}

impl core::fmt::Debug for WidgetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.0))
    }
}
