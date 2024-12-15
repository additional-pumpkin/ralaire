use core::str;
use vello_svg::usvg;

use crate::view::{View, ViewMarker};
use crate::widget::{self};

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
impl ViewMarker for ImageView {}

impl<State: 'static> View<State> for ImageView {
    type Element = widget::Image;
    fn build(&self) -> Self::Element {
        if let Ok(str) = str::from_utf8(&self.bytes) {
            if let Ok(_svg) = usvg::Tree::from_str(str, &usvg::Options::default()) {
                // TODO: implement svg support in the image widget and remove svg widget
                tracing::error!("FIXME: svg is not supported yet");
                // return widget::Svg::new(svg);
            }
        }
        widget::Image::new(&self.bytes)
    }

    fn rebuild(&self, old: &Self, _: &mut Self::Element) {
        if self.bytes != old.bytes {}
    }

    fn teardown(&self, _: &mut Self::Element) {}
}
