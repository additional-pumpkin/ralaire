use crate::Animation;

pub enum Command<Message> {
    Animate {
        animation: Animation,
        tick_message: Message,
        done_message: Message,
    },
}
