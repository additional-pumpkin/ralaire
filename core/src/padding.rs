// Copyright 2019 Héctor Ramón, Iced contributors
use crate::Size;

/// An amount of space to pad for each side of a box
///
/// You can leverage the `From` trait to build [`Padding`] conveniently:
///
/// ```
/// # use ralaire_core::Padding;
/// #
/// let padding = Padding::from(20.);              // 20px on all sides
/// let padding = Padding::from([10., 20.]);        // top/bottom, left/right
/// let padding = Padding::from([5., 10., 15., 20.]); // top, right, bottom, left
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Padding {
    /// Top padding
    pub top: f64,
    /// Right padding
    pub right: f64,
    /// Bottom padding
    pub bottom: f64,
    /// Left padding
    pub left: f64,
}

impl Padding {
    /// Padding of zero
    pub const ZERO: Padding = Padding {
        top: 0.0,
        right: 0.0,
        bottom: 0.0,
        left: 0.0,
    };

    /// Create a Padding that is equal on all sides
    pub const fn new(padding: f64) -> Padding {
        Padding {
            top: padding,
            right: padding,
            bottom: padding,
            left: padding,
        }
    }

    /// Returns the total amount of vertical [`Padding`].
    pub fn vertical(self) -> f64 {
        self.top + self.bottom
    }

    /// Returns the total amount of horizontal [`Padding`].
    pub fn horizontal(self) -> f64 {
        self.left + self.right
    }

    /// Fits the [`Padding`] between the provided `inner` and `outer` [`Size`].
    pub fn fit(self, inner: Size, outer: Size) -> Self {
        let available = (outer - inner).clamp(Size::ZERO, outer);
        let new_top = self.top.min(available.height / 2.);
        let new_left = self.left.min(available.width / 2.);

        Padding {
            top: new_top,
            bottom: self.bottom.min(available.height - new_top),
            left: new_left,
            right: self.right.min(available.width - new_left),
        }
    }
}

impl From<f64> for Padding {
    fn from(p: f64) -> Self {
        Padding {
            top: p,
            right: p,
            bottom: p,
            left: p,
        }
    }
}

impl From<[f64; 2]> for Padding {
    fn from(p: [f64; 2]) -> Self {
        Padding {
            top: p[0],
            right: p[1],
            bottom: p[0],
            left: p[1],
        }
    }
}

impl From<[f64; 4]> for Padding {
    fn from(p: [f64; 4]) -> Self {
        Padding {
            top: p[0],
            right: p[1],
            bottom: p[2],
            left: p[3],
        }
    }
}

impl PartialEq for Padding {
    fn eq(&self, other: &Self) -> bool {
        self.top == other.top
            && self.right == other.right
            && self.bottom == other.bottom
            && self.left == other.left
    }
}
