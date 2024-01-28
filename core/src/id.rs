// Copyright 2018 The xi-editor Authors, Héctor Ramón, Iced contributors
use std::{
    num::NonZeroU64,
    sync::atomic::{AtomicU64, Ordering},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct Id(NonZeroU64);

pub type IdPath = Vec<Id>;
static ID_COUNTER: AtomicU64 = AtomicU64::new(1);

impl Id {
    pub fn unique() -> Id {
        Id(NonZeroU64::new(ID_COUNTER.fetch_add(1, Ordering::Relaxed)).unwrap())
    }

    pub fn to_raw(self) -> u64 {
        self.0.into()
    }

    pub fn to_nonzero_raw(self) -> NonZeroU64 {
        self.0
    }
}
