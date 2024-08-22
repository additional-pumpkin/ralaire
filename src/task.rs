use crate::Animation;

pub struct Task<State> {
    pub(crate) tasks: Vec<InternalTask<State>>,
}
impl<State> Task<State> {
    pub fn none() -> Self {
        Self { tasks: vec![] }
    }
    pub fn animation(
        animation: Animation,
        tick_callback: impl Fn(&mut State) + Send + Sync + 'static,
        done_callback: impl Fn(&mut State) + Send + Sync + 'static,
    ) -> Self {
        Self {
            tasks: vec![InternalTask::Animate {
                animation,
                tick_callback: Box::new(tick_callback),
                done_callback: Box::new(done_callback),
            }],
        }
    }
    pub fn append(mut self, other: &mut Self) -> Self {
        self.tasks.append(&mut other.tasks);
        self
    }
}

pub(crate) enum InternalTask<State> {
    Animate {
        animation: Animation,
        tick_callback: Box<dyn Fn(&mut State)>,
        done_callback: Box<dyn Fn(&mut State)>,
    },
}
