use std::path::PathBuf;

use crate::view::View;
use crate::widget::{ImageWidget, SvgWidget, TextWidget, WidgetData};

pub struct ImageView {
    image_path: PathBuf,
}
impl ImageView {
    pub fn new(image_path: PathBuf) -> Self {
        Self { image_path }
    }
}

impl<Message> View<Message> for ImageView
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn build_widget(&self) -> WidgetData<Message> {
        if self
            .image_path
            .extension()
            .expect("Images without an extension are unsupported")
            .to_str()
            .unwrap()
            == "svg"
        {
            WidgetData::new(Box::new(SvgWidget::new(self.image_path.clone())))
        } else {
            WidgetData::new(Box::new(ImageWidget::new(self.image_path.clone())))
        }
    }

    fn change_widget(&self, widget_data: &mut WidgetData<Message>) {
        (*widget_data.widget)
            .as_any_mut()
            .downcast_mut::<TextWidget>()
            .unwrap();
        tracing::error!("Image changed, unimplemented!");
        widget_data.change_flags.layout = true;
        widget_data.change_flags.draw = true;
    }

    fn reconciliate(&self, old: &Box<dyn View<Message>>, widget: &mut WidgetData<Message>) {
        if self.image_path
            != (**old)
                .as_any()
                .downcast_ref::<ImageView>()
                .unwrap()
                .image_path
        {
            self.change_widget(widget)
        }
    }
}
