use crate::event;
use crate::widget::{Constraints, Widget};
use image::io::Reader as ImageReader;
use parley::FontContext;
use vello::peniko::kurbo::Size;
use vello::peniko::{Blob, Image};
use std::marker::PhantomData;
use std::path::Path;
use std::sync::Arc;
use vello::kurbo::Affine;

pub struct ImageWidget<Message> {
    image: Image,
    size: Size,
    phantom_message: PhantomData<Message>,
}

impl<Message> ImageWidget<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    pub fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let img = ImageReader::open(path)
            .unwrap()
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();
        let (w, h) = (img.width(), img.height());
        dbg!(w, h);
        let image = Image::new(
            Blob::new(Arc::new(img.to_rgba8().into_raw())),
            vello::peniko::Format::Rgba8,
            w,
            h,
        );
        let size = Size::new(image.width as f64, image.height as f64);
        Self {
            image,
            size,
            phantom_message: PhantomData,
        }
    }
    pub fn set_size(&mut self, size: Size) {
        self.size = size
    }
}

impl<Message> Widget<Message> for ImageWidget<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn paint(&self, scene: &mut vello::Scene) {
        scene.draw_image(&self.image.clone(), Affine::default());
    }

    fn debug_name(&self) -> &str {
        "image"
    }
    fn layout(&mut self, _constraints: Constraints, _font_cx: &mut FontContext) -> Size {
        // self.size = constraints.max_size;
        self.size
    }

    fn children(&self) -> Vec<&super::WidgetData<Message>> {
        vec![]
    }

    fn children_mut(&mut self) -> Vec<&mut super::WidgetData<Message>> {
        vec![]
    }

    fn event(
        &mut self,
        _event: event::WidgetEvent,
        _event_cx: &mut event::EventCx<Message>,
    ) -> event::Status {
        event::Status::Captured
    }

    fn set_hover(&mut self, _hover: bool) -> event::Status {
        event::Status::Ignored
    }
}
