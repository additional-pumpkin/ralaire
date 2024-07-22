use core::str;
use vello_svg::usvg;

use crate::view::View;
use crate::widget::{self, Text, WidgetData};

pub fn image(bytes: Vec<u8>) -> ImageView {
    ImageView::new(bytes)
}

pub struct ImageView {
    bytes: Vec<u8>,
}
impl ImageView {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
}

impl<Message> View<Message> for ImageView
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn build_widget(&self) -> WidgetData<Message> {
        if let Ok(str) = str::from_utf8(&self.bytes) {
            if let Ok(svg) = usvg::Tree::from_str(str, &usvg::Options::default()) {
                return WidgetData::new(Box::new(widget::Svg::new(svg)));
            }

        }
        WidgetData::new(Box::new(widget::Image::new(&self.bytes)))
        
    }

    fn change_widget(&self, widget_data: &mut WidgetData<Message>) {
        (*widget_data.inner)
            .as_any_mut()
            .downcast_mut::<Text>()
            .unwrap();
        tracing::error!("Image changed, unimplemented!");
        // widget_data.change_flags.needs_repaint = true;
    }

    fn reconciliate(&self, old: &Box<dyn View<Message>>, widget: &mut WidgetData<Message>) {
        if self.bytes
            != (**old)
                .as_any()
                .downcast_ref::<ImageView>()
                .unwrap()
                .bytes
        {
            self.change_widget(widget)
        }
    }
}
