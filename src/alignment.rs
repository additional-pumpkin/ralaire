/// The horizontal alignment of some resource.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Horizontal {
    /// Align left
    Left,

    /// Horizontally centered
    Center,

    /// Align right
    Right,
}

/// The vertical alignment of some resource.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Vertical {
    /// Align top
    Top,

    /// Vertically centered
    Center,

    /// Align bottom
    Bottom,
}
