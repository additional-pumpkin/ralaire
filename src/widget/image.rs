use crate::event;
use crate::widget::Widget;
use image::ImageReader;
use parley::FontContext;
use std::io::Cursor;
use std::marker::PhantomData;
use std::sync::Arc;
use vello::kurbo::Affine;
use vello::peniko::kurbo::Size;
use vello::peniko::{Blob, Image as PenikoImage};

pub struct Image<Message> {
    image: PenikoImage,
    size: Size,
    phantom_message: PhantomData<Message>,
}

impl<Message> Image<Message>
where
    Message: Clone + core::fmt::Debug + 'static,
{
    pub fn new(bytes: &[u8]) -> Self
    {
        let img = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();
        let (w, h) = (img.width(), img.height());
        dbg!(w, h);
        let image = PenikoImage::new(
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

impl<Message> Widget<Message> for Image<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn paint(&mut self, scene: &mut vello::Scene) {
        scene.draw_image(&self.image.clone(), Affine::default());
    }

    fn debug_name(&self) -> &str {
        "image"
    }
    fn layout(&mut self, _size_hint: Size, _font_cx: &mut FontContext) -> Size {
        // self.size = size_hint;
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
