use crate::Animation;

pub struct Command<Message> {
    pub(crate) tasks: Vec<Task<Message>>,
}
impl<Message> Command<Message> {
    pub fn none() -> Self {
        Self { tasks: vec![] }
    }
    pub fn animation(animation: Animation, tick_message: Message, done_message: Message) -> Self {
        Self {
            tasks: vec![Task::Animate {
                animation,
                tick_message,
                done_message,
            }],
        }
    }
    pub fn append(mut self, other: &mut Self) -> Self {
        self.tasks.append(&mut other.tasks);
        self
    }
}

pub(crate) enum Task<Message> {
    Animate {
        animation: Animation,
        tick_message: Message,
        done_message: Message,
    },
}
