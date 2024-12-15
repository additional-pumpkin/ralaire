use crate::{
    view::{container, Container, Header, View},
    widget::{self, Widget},
};

// TODO: Come up with better names
pub fn window<Content>(
    content: Content,
    title: String,
) -> Window<String, Container<String>, String, Content> {
    let header = Header::new(
        "left".to_owned(),
        container(title.clone()),
        "right".to_owned(),
    );
    Window::new(header, content, title)
}
pub struct Window<Left, Middle, Right, Content> {
    header: Header<Left, Middle, Right>,
    content: Content,
    title: String,
}

impl<Left, Middle, Right, Content> Window<Left, Middle, Right, Content> {
    pub fn new(header: Header<Left, Middle, Right>, content: Content, title: String) -> Self {
        Self {
            header,
            content,
            title,
        }
    }
}
impl<State: 'static, Left, Middle, Right, Content> View<State>
    for Window<Left, Middle, Right, Content>
where
    Left: View<State>,
    Middle: View<State>,
    Right: View<State>,
    Content: View<State>,
    Left::Element: Widget<State>,
    Middle::Element: Widget<State>,
    Right::Element: Widget<State>,
    Content::Element: Widget<State>,
{
    type Element = widget::Window<State>;
    fn build(&self) -> Self::Element {
        let header = self.header.build();
        let content = self.content.build();
        widget::Window::new(header, content, self.title.clone())
    }
    fn rebuild(&self, old: &Self, element: &mut Self::Element) {
        if self.title != old.title {
            element.title.clone_from(&self.title);
        }
        self.header.rebuild(
            &old.header,
            (*element.header.inner)
                .as_any_mut()
                .downcast_mut::<widget::Header<State>>()
                .unwrap(),
        );
        self.content.rebuild(
            &old.content,
            (*element.content.inner)
                .as_any_mut()
                .downcast_mut::<Content::Element>()
                .unwrap(),
        );
    }

    fn teardown(&self, element: &mut Self::Element) {
        self.header.teardown(
            (*element.header.inner)
                .as_any_mut()
                .downcast_mut::<widget::Header<State>>()
                .unwrap(),
        );
        self.content.teardown(
            (*element.content.inner)
                .as_any_mut()
                .downcast_mut::<Content::Element>()
                .unwrap(),
        );
    }
}
