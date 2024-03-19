use crate::event::ResizeDirection;
#[derive(Debug, Clone)]
pub enum InternalMessage {
    DragResizeWindow(ResizeDirection),
    DragMoveWindow,
}
#[derive(Debug, Clone)]
pub enum AppMessage<UserMessage: core::fmt::Debug + Clone + 'static> {
    Internal(InternalMessage),
    User(UserMessage),
}
