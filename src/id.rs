use core::{
    num::NonZeroU64,
    sync::atomic::{AtomicU64, Ordering},
};
extern crate alloc;
use alloc::vec::Vec;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub struct WidgetId(NonZeroU64);

pub type WidgetIdPath = Vec<WidgetId>;

impl WidgetId {
    pub fn unique() -> WidgetId {
        static WIDGET_ID_COUNTER: AtomicU64 = AtomicU64::new(1);
        WidgetId(NonZeroU64::new(WIDGET_ID_COUNTER.fetch_add(1, Ordering::Relaxed)).unwrap())
    }

    pub fn to_raw(self) -> u64 {
        self.0.into()
    }

    pub fn to_nonzero_raw(self) -> NonZeroU64 {
        self.0
    }
}

impl core::fmt::Display for WidgetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.0))
    }
}

impl core::fmt::Debug for WidgetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.0))
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct AnimationId(NonZeroU64);

impl AnimationId {
    pub fn unique() -> AnimationId {
        static ANIMATION_ID_COUNTER: AtomicU64 = AtomicU64::new(1);
        AnimationId(NonZeroU64::new(ANIMATION_ID_COUNTER.fetch_add(1, Ordering::Relaxed)).unwrap())
    }

    pub fn to_raw(self) -> u64 {
        self.0.into()
    }

    pub fn to_nonzero_raw(self) -> NonZeroU64 {
        self.0
    }
}
