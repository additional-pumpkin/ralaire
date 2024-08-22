use core::{
    f64::consts::PI,
    sync::atomic::{AtomicU64, Ordering},
};
use std::{num::NonZeroU64, sync::Arc};

#[derive(Debug, Clone)]
pub struct Animation {
    id: AnimationId,
    value: Arc<AtomicU64>,
    update_interval: core::time::Duration,
    duration: core::time::Duration,
    direction: AnimationDirection,
    easing_curve: InternalEasingCurve,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationDirection {
    Forward,
    Backward,
}

// https://easings.net/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EasingCurve {
    Linear,
    EaseInSine,
    EaseOutSine,
    EaseInOutSine,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInQuart,
    EaseOutQuart,
    EaseInOutQuart,
    EaseInQuint,
    EaseOutQuint,
    EaseInOutQuint,
    EaseInExpo,
    EaseOutExpo,
    EaseInOutExpo,
    EaseInCirc,
    EaseOutCirc,
    EaseInOutCirc,
    EaseInBack,
    EaseOutBack,
    EaseInOutBack,
    EaseInElastic,
    EaseOutElastic,
    EaseInOutElastic,
    EaseInBounce,
    EaseOutBounce,
    EaseInOutBounce,
}
#[derive(Debug, Clone, Copy, PartialEq)]
enum InternalEasingCurve {
    Predefined(EasingCurve),
    Custom(fn(f64) -> f64),
}
impl Animation {
    pub fn new(direction: AnimationDirection, duration: core::time::Duration) -> Self {
        match direction {
            AnimationDirection::Forward => Self {
                id: AnimationId::unique(),
                value: Arc::new(AtomicU64::new(0)),
                update_interval: core::time::Duration::from_millis(16),
                duration,
                direction,
                easing_curve: InternalEasingCurve::Predefined(EasingCurve::EaseInOutCubic),
            },
            AnimationDirection::Backward => Self {
                id: AnimationId::unique(),
                value: Arc::new(AtomicU64::new(duration.as_millis() as u64 / 16)),
                update_interval: core::time::Duration::from_millis(16),
                duration,
                direction,
                easing_curve: InternalEasingCurve::Predefined(EasingCurve::EaseInOutCubic),
            },
        }
    }
    pub fn with_easing(mut self, curve: EasingCurve) -> Self {
        self.easing_curve = InternalEasingCurve::Predefined(curve);
        self
    }
    pub fn with_custom_easing(mut self, custom_easing: fn(f64) -> f64) -> Self {
        self.easing_curve = InternalEasingCurve::Custom(custom_easing);
        self
    }
    pub fn increment(&mut self) {
        self.value.fetch_add(1, Ordering::Release);
    }
    pub fn decrement(&mut self) {
        self.value.fetch_sub(1, Ordering::Release);
    }
    pub fn raw_value(&self) -> f64 {
        self.value.load(Ordering::Acquire) as f64 / (self.duration / 16).as_millis() as f64
    }
    pub fn value(&self) -> f64 {
        let x = self.raw_value();
        fn ease_out_bounce(x: f64) -> f64 {
            if x < 1. / 2.75 {
                7.5625 * x * x
            } else if x < 2. / 2.75 {
                7.5625 * (x - 1.5 / 2.75) * (x - 1.5 / 2.75) + 0.75
            } else if x < 2.5 / 2.75 {
                7.5625 * (x - 2.25 / 2.75) * (x - 2.25 / 2.75) + 0.9375
            } else {
                7.5625 * (x - 2.625 / 2.75) * (x - 2.625 / 2.75) + 0.984375
            }
        }
        match self.easing_curve {
            InternalEasingCurve::Predefined(curve) => match curve {
                EasingCurve::Linear => x,
                EasingCurve::EaseInSine => 1. - ((x * PI) / 2.).cos(),
                EasingCurve::EaseOutSine => ((x * PI) / 2.).sin(),
                EasingCurve::EaseInOutSine => -((PI * x).cos() - 1.) / 2.,
                EasingCurve::EaseInQuad => x * x,
                EasingCurve::EaseOutQuad => 1. - (1. - x) * (1. - x),
                EasingCurve::EaseInOutQuad => {
                    if x < 0.5 {
                        2. * x * x
                    } else {
                        1. - (-2. * x + 2.).powi(2) / 2.
                    }
                }
                EasingCurve::EaseInCubic => x * x * x,
                EasingCurve::EaseOutCubic => 1. - (1. - x).powi(3),
                EasingCurve::EaseInOutCubic => {
                    if x < 0.5 {
                        4. * x * x * x
                    } else {
                        1. - (-2. * x + 2.).powi(3) / 2.
                    }
                }
                EasingCurve::EaseInQuart => x * x * x * x,
                EasingCurve::EaseOutQuart => 1. - (1. - x).powi(4),
                EasingCurve::EaseInOutQuart => {
                    if x < 0.5 {
                        8. * x * x * x * x
                    } else {
                        1. - (-2. * x + 2.).powi(4) / 2.
                    }
                }
                EasingCurve::EaseInQuint => x * x * x * x * x,
                EasingCurve::EaseOutQuint => 1. - (1. - x).powi(5),
                EasingCurve::EaseInOutQuint => {
                    if x < 0.5 {
                        16. * x * x * x * x * x
                    } else {
                        1. - (-2. * x + 2.).powi(5) / 2.
                    }
                }
                EasingCurve::EaseInExpo => {
                    if x == 0. {
                        0.
                    } else {
                        2_f64.powf(10. * x - 10.)
                    }
                }
                EasingCurve::EaseOutExpo => {
                    if x == 1. {
                        1.
                    } else {
                        1. - 2_f64.powf(-10. * x)
                    }
                }
                EasingCurve::EaseInOutExpo => {
                    if x == 0. {
                        0.
                    } else if x == 1. {
                        1.
                    } else if x < 0.5 {
                        2_f64.powf(20. * x - 10.) / 2.
                    } else {
                        (2. - 2_f64.powf(-20. * x + 10.)) / 2.
                    }
                }
                EasingCurve::EaseInCirc => 1. - (1. - x * x).sqrt(),
                EasingCurve::EaseOutCirc => (1. - (x - 1.) * (x - 1.)).sqrt(),
                EasingCurve::EaseInOutCirc => {
                    if x < 0.5 {
                        (1. - (1. - 4. * x * x)).sqrt() / 2.
                    } else {
                        ((1. - (-2. * x + 2.) * (-2. * x + 2.)).sqrt() + 1.) / 2.
                    }
                }
                EasingCurve::EaseInBack => 2.70158 * x * x * x - 1.70158 * x * x,
                EasingCurve::EaseOutBack => {
                    1. + 2.70158 * (x - 1.).powi(3) + 1.70158 * (x - 1.).powi(2)
                }

                EasingCurve::EaseInOutBack => {
                    if x < 0.5 {
                        (4. * x * x * ((2.5949095 + 1.) * 2. * x - 2.5949095)) / 2.
                    } else {
                        ((2. * x - 2.).powi(2) * ((2.5949095 + 1.) * (x * 2. - 2.) + 2.5949095)
                            + 2.)
                            / 2.
                    }
                }
                EasingCurve::EaseInElastic => {
                    if x == 0. {
                        0.
                    } else if x == 1. {
                        1.
                    } else {
                        -2_f64.powf(10. * x - 10.) * ((x * 10. - 10.75).sin() * 2. * PI / 3.)
                    }
                }
                EasingCurve::EaseOutElastic => {
                    if x == 0. {
                        0.
                    } else if x == 1. {
                        1.
                    } else {
                        2_f64.powf(-10. * x) * ((x * 10. - 0.75) * 2. * PI / 3.).sin() + 1.
                    }
                }
                EasingCurve::EaseInOutElastic => {
                    if x == 0. {
                        0.
                    } else if x == 1. {
                        1.
                    } else if x < 0.5 {
                        -(2_f64.powf(20. * x - 10.) * ((20. * x - 11.125) * 2. * PI / 4.5).sin())
                            / 2.
                    } else {
                        (2_f64.powf(-20. * x + 10.) * ((20. * x - 11.125) * 2. * PI / 4.5).sin())
                            / 2.
                            + 1.
                    }
                }
                EasingCurve::EaseInBounce => 1. - ease_out_bounce(1. - x),
                EasingCurve::EaseOutBounce => ease_out_bounce(x),
                EasingCurve::EaseInOutBounce => {
                    if x < 0.5 {
                        (1. - ease_out_bounce(1. - 2. * x)) / 2.
                    } else {
                        (1. + ease_out_bounce(2. * x - 1.)) / 2.
                    }
                }
            },
            InternalEasingCurve::Custom(custom) => custom(x),
        }
    }
    pub fn id(&self) -> AnimationId {
        self.id
    }
    pub fn update_interval(&self) -> core::time::Duration {
        self.update_interval
    }
    pub fn duration(&self) -> core::time::Duration {
        self.duration
    }
    pub fn direction(&self) -> AnimationDirection {
        self.direction
    }
}

impl PartialEq<Animation> for Animation {
    fn eq(&self, other: &Animation) -> bool {
        self.id == other.id
            && self.update_interval == other.update_interval
            && self.duration == other.duration
            && self.direction == other.direction
            && self.easing_curve == other.easing_curve
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
