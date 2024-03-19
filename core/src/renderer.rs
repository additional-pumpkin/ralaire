use crate::{Affine, BezPath, BlendMode, Brush, DebugLayer, Shape, Size, Stroke, WindowSize};
extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;
use parley::Layout;
use peniko::kurbo::RoundedRect;

#[derive(Clone)]
pub enum TextLayout {
    ParleyLayout(Layout<Brush>),
}

impl core::fmt::Debug for TextLayout {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ParleyLayout(arg0) => f
                .debug_tuple("ParleyLayout")
                .field(&Size::new(arg0.width() as f64, arg0.height() as f64))
                .finish(),
        }
    }
}

pub struct RenderCx {
    pub(crate) command_stack: Vec<RenderCommand>,
}

impl RenderCx {
    pub fn new() -> Self {
        RenderCx {
            command_stack: vec![],
        }
    }

    pub fn get_command_lists(&self) -> Vec<Vec<RenderCommand>> {
        let mut command_lists: Vec<Vec<RenderCommand>> = vec![];
        let mut push_stack = vec![];
        let _: Vec<()> = self
            .command_stack
            .clone()
            .into_iter()
            .map(|command| match command {
                RenderCommand::PushLayer {
                    blend: _,
                    transform: _,
                    bounds: _,
                } => {
                    if command_lists.last().is_some() {
                        for _ in &push_stack {
                            command_lists
                                .last_mut()
                                .unwrap()
                                .push(RenderCommand::PopLayer);
                        }
                    }
                    push_stack.push(command);
                    command_lists.push(push_stack.clone());
                }
                RenderCommand::PopLayer => {
                    push_stack.pop();
                    command_lists.last_mut().unwrap().push(command);
                }
                _ => command_lists.last_mut().unwrap().push(command),
            })
            .collect();
        command_lists
    }

    /// Pushes a new layer bound by the [`Shape`] and using the specified [`BlendMode`].
    pub fn push_layer(&mut self, blend: BlendMode, transform: Affine, bounds: RoundedRect) {
        self.command_stack.push(RenderCommand::PushLayer {
            blend,
            transform,
            bounds,
        });
    }
    /// Pops the current layer
    pub fn pop_layer(&mut self) {
        self.command_stack.push(RenderCommand::PopLayer);
    }
    /// Fills a [`Shape`] with the provided [`Color`].
    pub fn fill_shape<'be>(
        &mut self,
        transform: Affine,
        shape: &impl Shape,
        background: impl Into<Brush>,
    ) {
        self.command_stack.push(RenderCommand::FillShape {
            transform,
            shape: shape.into_path(0.1),
            brush: background.into(),
        });
    }

    /// Strokes a [`Shape`] with the provided [`Color`].
    pub fn stroke_shape<'be>(
        &mut self,
        style: Stroke,
        transform: Affine,
        shape: &impl Shape,
        color: impl Into<Brush>,
    ) {
        self.command_stack.push(RenderCommand::StrokeShape {
            style,
            transform,
            shape: shape.into_path(0.1),
            brush: color.into(),
        });
    }

    /// Draws an svg applying the specified transform.
    pub fn draw_svg(&mut self, _transform: Affine, _svg_data: &[u8]) {
        todo!()
    }

    /// Draws the specified text using the specified [`Font`].
    pub fn draw_text(&mut self, layout: TextLayout) {
        self.command_stack.push(RenderCommand::DrawText { layout });
    }

    /// Clears all commands in [`Renderer`].
    pub fn clear(&mut self) {
        self.command_stack.clear();
    }
}

#[derive(Debug, Clone)]
pub enum RenderCommand {
    PushLayer {
        blend: BlendMode,
        transform: Affine,
        bounds: RoundedRect,
    },
    PopLayer,
    FillShape {
        transform: Affine,
        shape: BezPath,
        brush: Brush,
    },
    StrokeShape {
        style: Stroke,
        transform: Affine,
        shape: BezPath,
        brush: Brush,
    },
    DrawSvg {
        transfomr: Affine,
        svg_data: Vec<u8>,
    },
    DrawText {
        layout: TextLayout,
    },
}

pub trait Renderer {
    /// Takes a list of [`RenderCommand`] from each widget and renders them in order.
    fn render(&mut self, command_lists: Vec<Vec<RenderCommand>>, debug: &mut DebugLayer);

    fn resize(&mut self, new_size: WindowSize);
}
