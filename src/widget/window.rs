use super::WidgetData;
use crate::event::mouse::MouseButton;
use crate::event::WidgetEvent;
use crate::widget::Widget;
use crate::{event, InternalMessage, WidgetId};
use core::f64::consts::PI;
use parley::FontContext;
use vello::peniko::kurbo::{Affine, Circle, Point, Rect, RoundedRect, Shape, Size};
use vello::peniko::{BlendMode, Brush, Color, Fill, Gradient};
use winit::window::ResizeDirection;
const CORNER_RADIUS: f64 = 12.;
const SHADOW_WIDTH: f64 = 15.;
const HEADER_BAR_HEIGHT: f64 = 46.;
const SHADOW_COLOR: Color = Color::rgba8(0, 0, 0, 100);
const SHADOW_FADE_COLOR: Color = Color::rgba8(0, 0, 0, 0);

fn linear_component(u: f64) -> f64 {
    if u < 0.04045 {
        u / 12.92
    } else {
        ((u + 0.055) / 1.055).powf(2.4)
    }
}
fn gamma_component(u: f64) -> f64 {
    if u < 0.0031308 {
        12.92 * u
    } else {
        1.055 * u.powf(1.0 / 2.4) - 0.055
    }
}

fn interpolate(start: f64, end: f64, factor: f64) -> f64 {
    gamma_component(
        (linear_component(end) - linear_component(start)) * factor + linear_component(start),
    )
}
#[derive(Debug)]
pub struct Window<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    // TODO: remove this
    pub id: WidgetId,
    bounds: RoundedRect,
    size: Size, // includes shadows
    header: WidgetData<Message>,
    content: WidgetData<Message>,
    title: String,
}

impl<Message> Window<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    pub fn new(header: WidgetData<Message>, content: WidgetData<Message>, title: String) -> Self {
        Window {
            id: WidgetId::unique(),
            bounds: Rect::ZERO.to_rounded_rect(CORNER_RADIUS),
            size: Size::ZERO,
            header,
            content,
            title,
        }
    }
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
    pub fn header(&mut self) -> &mut WidgetData<Message> {
        &mut self.header
    }
    pub fn content(&mut self) -> &mut WidgetData<Message> {
        &mut self.content
    }
}
impl<Message> Widget<Message> for Window<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    fn debug_name(&self) -> &str {
        "window"
    }
    // TODO: Support SSD on wayland
    // TODO: Figure out what to do for other platforms
    fn paint(&mut self, scene: &mut vello::Scene) {
        // Normally shadows are implemented with blur, vello doesn't support it yet so
        // here we approximate the gaussian function exp(-8x^2) by using 11 color points
        // and linear interpolation between the SHADOW_COLOR and SHADOW_FADE_COLOR
        let shadow_color_stops = [
            (0., SHADOW_COLOR),
            (
                0.1,
                Color::rgba(
                    interpolate(
                        SHADOW_COLOR.r as f64 / 255.,
                        SHADOW_FADE_COLOR.r as f64 / 255.,
                        1.0 - 0.9231,
                    ),
                    interpolate(
                        SHADOW_COLOR.g as f64 / 255.,
                        SHADOW_FADE_COLOR.g as f64 / 255.,
                        1.0 - 0.9231,
                    ),
                    interpolate(
                        SHADOW_COLOR.b as f64 / 255.,
                        SHADOW_FADE_COLOR.b as f64 / 255.,
                        1.0 - 0.9231,
                    ),
                    interpolate(
                        SHADOW_COLOR.a as f64 / 255.,
                        SHADOW_FADE_COLOR.a as f64 / 255.,
                        1.0 - 0.9231,
                    ),
                ),
            ),
            (
                0.2,
                Color::rgba(
                    interpolate(
                        SHADOW_COLOR.r as f64 / 255.,
                        SHADOW_FADE_COLOR.r as f64 / 255.,
                        1.0 - 0.7261,
                    ),
                    interpolate(
                        SHADOW_COLOR.g as f64 / 255.,
                        SHADOW_FADE_COLOR.g as f64 / 255.,
                        1.0 - 0.7261,
                    ),
                    interpolate(
                        SHADOW_COLOR.b as f64 / 255.,
                        SHADOW_FADE_COLOR.b as f64 / 255.,
                        1.0 - 0.7261,
                    ),
                    interpolate(
                        SHADOW_COLOR.a as f64 / 255.,
                        SHADOW_FADE_COLOR.a as f64 / 255.,
                        1.0 - 0.7261,
                    ),
                ),
            ),
            (
                0.3,
                Color::rgba(
                    interpolate(
                        SHADOW_COLOR.r as f64 / 255.,
                        SHADOW_FADE_COLOR.r as f64 / 255.,
                        1.0 - 0.4868,
                    ),
                    interpolate(
                        SHADOW_COLOR.g as f64 / 255.,
                        SHADOW_FADE_COLOR.g as f64 / 255.,
                        1.0 - 0.4868,
                    ),
                    interpolate(
                        SHADOW_COLOR.b as f64 / 255.,
                        SHADOW_FADE_COLOR.b as f64 / 255.,
                        1.0 - 0.4868,
                    ),
                    interpolate(
                        SHADOW_COLOR.a as f64 / 255.,
                        SHADOW_FADE_COLOR.a as f64 / 255.,
                        1.0 - 0.4868,
                    ),
                ),
            ),
            (
                0.4,
                Color::rgba(
                    interpolate(
                        SHADOW_COLOR.r as f64 / 255.,
                        SHADOW_FADE_COLOR.r as f64 / 255.,
                        1.0 - 0.2780,
                    ),
                    interpolate(
                        SHADOW_COLOR.g as f64 / 255.,
                        SHADOW_FADE_COLOR.g as f64 / 255.,
                        1.0 - 0.2780,
                    ),
                    interpolate(
                        SHADOW_COLOR.b as f64 / 255.,
                        SHADOW_FADE_COLOR.b as f64 / 255.,
                        1.0 - 0.2780,
                    ),
                    interpolate(
                        SHADOW_COLOR.a as f64 / 255.,
                        SHADOW_FADE_COLOR.a as f64 / 255.,
                        1.0 - 0.2780,
                    ),
                ),
            ),
            (
                0.5,
                Color::rgba(
                    interpolate(
                        SHADOW_COLOR.r as f64 / 255.,
                        SHADOW_FADE_COLOR.r as f64 / 255.,
                        1.0 - 0.1353,
                    ),
                    interpolate(
                        SHADOW_COLOR.g as f64 / 255.,
                        SHADOW_FADE_COLOR.g as f64 / 255.,
                        1.0 - 0.1353,
                    ),
                    interpolate(
                        SHADOW_COLOR.b as f64 / 255.,
                        SHADOW_FADE_COLOR.b as f64 / 255.,
                        1.0 - 0.1353,
                    ),
                    interpolate(
                        SHADOW_COLOR.a as f64 / 255.,
                        SHADOW_FADE_COLOR.a as f64 / 255.,
                        1.0 - 0.1353,
                    ),
                ),
            ),
            (
                0.6,
                Color::rgba(
                    interpolate(
                        SHADOW_COLOR.r as f64 / 255.,
                        SHADOW_FADE_COLOR.r as f64 / 255.,
                        1.0 - 0.056135,
                    ),
                    interpolate(
                        SHADOW_COLOR.g as f64 / 255.,
                        SHADOW_FADE_COLOR.g as f64 / 255.,
                        1.0 - 0.056135,
                    ),
                    interpolate(
                        SHADOW_COLOR.b as f64 / 255.,
                        SHADOW_FADE_COLOR.b as f64 / 255.,
                        1.0 - 0.056135,
                    ),
                    interpolate(
                        SHADOW_COLOR.a as f64 / 255.,
                        SHADOW_FADE_COLOR.a as f64 / 255.,
                        1.0 - 0.056135,
                    ),
                ),
            ),
            (
                0.7,
                Color::rgba(
                    interpolate(
                        SHADOW_COLOR.r as f64 / 255.,
                        SHADOW_FADE_COLOR.r as f64 / 255.,
                        1.0 - 0.019841,
                    ),
                    interpolate(
                        SHADOW_COLOR.g as f64 / 255.,
                        SHADOW_FADE_COLOR.g as f64 / 255.,
                        1.0 - 0.019841,
                    ),
                    interpolate(
                        SHADOW_COLOR.b as f64 / 255.,
                        SHADOW_FADE_COLOR.b as f64 / 255.,
                        1.0 - 0.019841,
                    ),
                    interpolate(
                        SHADOW_COLOR.a as f64 / 255.,
                        SHADOW_FADE_COLOR.a as f64 / 255.,
                        1.0 - 0.019841,
                    ),
                ),
            ),
            (
                0.8,
                Color::rgba(
                    interpolate(
                        SHADOW_COLOR.r as f64 / 255.,
                        SHADOW_FADE_COLOR.r as f64 / 255.,
                        1.0 - 5.9760e-03,
                    ),
                    interpolate(
                        SHADOW_COLOR.g as f64 / 255.,
                        SHADOW_FADE_COLOR.g as f64 / 255.,
                        1.0 - 5.9760e-03,
                    ),
                    interpolate(
                        SHADOW_COLOR.b as f64 / 255.,
                        SHADOW_FADE_COLOR.b as f64 / 255.,
                        1.0 - 5.9760e-03,
                    ),
                    interpolate(
                        SHADOW_COLOR.a as f64 / 255.,
                        SHADOW_FADE_COLOR.a as f64 / 255.,
                        1.0 - 5.9760e-03,
                    ),
                ),
            ),
            (
                0.9,
                Color::rgba(
                    interpolate(
                        SHADOW_COLOR.r as f64 / 255.,
                        SHADOW_FADE_COLOR.r as f64 / 255.,
                        1.0 - 1.5338e-03,
                    ),
                    interpolate(
                        SHADOW_COLOR.g as f64 / 255.,
                        SHADOW_FADE_COLOR.g as f64 / 255.,
                        1.0 - 1.5338e-03,
                    ),
                    interpolate(
                        SHADOW_COLOR.b as f64 / 255.,
                        SHADOW_FADE_COLOR.b as f64 / 255.,
                        1.0 - 1.5338e-03,
                    ),
                    interpolate(
                        SHADOW_COLOR.a as f64 / 255.,
                        SHADOW_FADE_COLOR.a as f64 / 255.,
                        1.0 - 1.5338e-03,
                    ),
                ),
            ),
            (
                1.0,
                Color::rgba(
                    interpolate(
                        SHADOW_COLOR.r as f64 / 255.,
                        SHADOW_FADE_COLOR.r as f64 / 255.,
                        1.0 - 3.3546e-04,
                    ),
                    interpolate(
                        SHADOW_COLOR.g as f64 / 255.,
                        SHADOW_FADE_COLOR.g as f64 / 255.,
                        1.0 - 3.3546e-04,
                    ),
                    interpolate(
                        SHADOW_COLOR.b as f64 / 255.,
                        SHADOW_FADE_COLOR.b as f64 / 255.,
                        1.0 - 3.3546e-04,
                    ),
                    interpolate(
                        SHADOW_COLOR.a as f64 / 255.,
                        SHADOW_FADE_COLOR.a as f64 / 255.,
                        1.0 - 3.3546e-04,
                    ),
                ),
            ),
        ];
        // top shadow
        scene.fill(
            Fill::NonZero,
            Affine::default(),
            &Brush::Gradient(
                Gradient::new_linear(
                    Point::new(
                        (self.size.width - (SHADOW_WIDTH + CORNER_RADIUS) * 2.) / 2.,
                        SHADOW_WIDTH + CORNER_RADIUS,
                    ),
                    Point::new(
                        (self.size.width - (SHADOW_WIDTH + CORNER_RADIUS) * 2.) / 2.,
                        0.,
                    ),
                )
                .with_stops(shadow_color_stops),
            ),
            None,
            &Rect::from_origin_size(
                Point::new(SHADOW_WIDTH + CORNER_RADIUS, 0.),
                Size::new(
                    self.size.width - (SHADOW_WIDTH + CORNER_RADIUS) * 2.,
                    SHADOW_WIDTH + CORNER_RADIUS,
                ),
            ),
        );
        // bottom shadow
        scene.fill(
            Fill::NonZero,
            Affine::default(),
            &Brush::Gradient(
                Gradient::new_linear(
                    Point::new(
                        (self.size.width - (SHADOW_WIDTH + CORNER_RADIUS) * 2.) / 2.,
                        self.size.height - (SHADOW_WIDTH + CORNER_RADIUS),
                    ),
                    Point::new(
                        (self.size.width - (SHADOW_WIDTH + CORNER_RADIUS) * 2.) / 2.,
                        self.size.height,
                    ),
                )
                .with_stops(shadow_color_stops),
            ),
            None,
            &Rect::from_origin_size(
                Point::new(
                    SHADOW_WIDTH + CORNER_RADIUS,
                    self.size.height - (SHADOW_WIDTH + CORNER_RADIUS),
                ),
                Size::new(
                    self.size.width - (SHADOW_WIDTH + CORNER_RADIUS) * 2.,
                    SHADOW_WIDTH + CORNER_RADIUS,
                ),
            ),
        );
        // right shadow
        scene.fill(
            Fill::NonZero,
            Affine::default(),
            &Brush::Gradient(
                Gradient::new_linear(
                    Point::new(
                        self.size.width - (SHADOW_WIDTH + CORNER_RADIUS),
                        (self.size.height - (SHADOW_WIDTH + CORNER_RADIUS) * 2.) / 2.,
                    ),
                    Point::new(
                        self.size.width,
                        (self.size.height - (SHADOW_WIDTH + CORNER_RADIUS) * 2.) / 2.,
                    ),
                )
                .with_stops(shadow_color_stops),
            ),
            None,
            &Rect::from_origin_size(
                Point::new(
                    self.size.width - (SHADOW_WIDTH + CORNER_RADIUS),
                    SHADOW_WIDTH + CORNER_RADIUS,
                ),
                Size::new(
                    SHADOW_WIDTH + CORNER_RADIUS,
                    self.size.height - (SHADOW_WIDTH + CORNER_RADIUS) * 2.,
                ),
            ),
        );
        // left shadow
        scene.fill(
            Fill::NonZero,
            Affine::default(),
            &Brush::Gradient(
                Gradient::new_linear(
                    Point::new(
                        SHADOW_WIDTH + CORNER_RADIUS,
                        (self.size.height - (SHADOW_WIDTH + CORNER_RADIUS) * 2.) / 2.,
                    ),
                    Point::new(
                        0.,
                        (self.size.height - (SHADOW_WIDTH + CORNER_RADIUS) * 2.) / 2.,
                    ),
                )
                .with_stops(shadow_color_stops),
            ),
            None,
            &Rect::from_origin_size(
                Point::new(0., SHADOW_WIDTH + CORNER_RADIUS),
                Size::new(
                    SHADOW_WIDTH + CORNER_RADIUS,
                    self.size.height - (SHADOW_WIDTH + CORNER_RADIUS) * 2.,
                ),
            ),
        );

        // corner shadows

        // top left
        scene.fill(
            Fill::NonZero,
            Affine::default(),
            &Brush::Gradient(
                Gradient::new_radial(
                    Point::new(SHADOW_WIDTH + CORNER_RADIUS, SHADOW_WIDTH + CORNER_RADIUS),
                    (SHADOW_WIDTH + CORNER_RADIUS) as f32,
                )
                .with_stops(shadow_color_stops),
            ),
            None,
            &Circle::new(
                Point::new(SHADOW_WIDTH + CORNER_RADIUS, SHADOW_WIDTH + CORNER_RADIUS),
                SHADOW_WIDTH + CORNER_RADIUS,
            )
            .segment(0., PI, 1. / 2. * PI),
        );
        // top right
        scene.fill(
            Fill::NonZero,
            Affine::default(),
            &Brush::Gradient(
                Gradient::new_radial(
                    Point::new(
                        self.size.width - (SHADOW_WIDTH + CORNER_RADIUS),
                        SHADOW_WIDTH + CORNER_RADIUS,
                    ),
                    (SHADOW_WIDTH + CORNER_RADIUS) as f32,
                )
                .with_stops(shadow_color_stops),
            ),
            None,
            &Circle::new(
                Point::new(
                    self.size.width - (SHADOW_WIDTH + CORNER_RADIUS),
                    SHADOW_WIDTH + CORNER_RADIUS,
                ),
                SHADOW_WIDTH + CORNER_RADIUS,
            )
            .segment(0., 3. / 2. * PI, 1. / 2. * PI),
        );
        // bottom right
        scene.fill(
            Fill::NonZero,
            Affine::default(),
            &Brush::Gradient(
                Gradient::new_radial(
                    Point::new(
                        self.size.width - (SHADOW_WIDTH + CORNER_RADIUS),
                        self.size.height - (SHADOW_WIDTH + CORNER_RADIUS),
                    ),
                    (SHADOW_WIDTH + CORNER_RADIUS) as f32,
                )
                .with_stops(shadow_color_stops),
            ),
            None,
            &Circle::new(
                Point::new(
                    self.size.width - (SHADOW_WIDTH + CORNER_RADIUS),
                    self.size.height - (SHADOW_WIDTH + CORNER_RADIUS),
                ),
                SHADOW_WIDTH + CORNER_RADIUS,
            )
            .segment(0., 0., 1. / 2. * PI),
        );
        // bottom left
        scene.fill(
            Fill::NonZero,
            Affine::default(),
            &Brush::Gradient(
                Gradient::new_radial(
                    Point::new(
                        SHADOW_WIDTH + CORNER_RADIUS,
                        self.size.height - (SHADOW_WIDTH + CORNER_RADIUS),
                    ),
                    (SHADOW_WIDTH + CORNER_RADIUS) as f32,
                )
                .with_stops(shadow_color_stops),
            ),
            None,
            &Circle::new(
                Point::new(
                    SHADOW_WIDTH + CORNER_RADIUS,
                    self.size.height - (SHADOW_WIDTH + CORNER_RADIUS),
                ),
                SHADOW_WIDTH + CORNER_RADIUS,
            )
            .segment(0., 1. / 2. * PI, 1. / 2. * PI),
        );

        scene.fill(
            Fill::NonZero,
            Affine::default(),
            Color::WHITE,
            None,
            &self.bounds,
        );
        scene.push_layer(BlendMode::default(), 1.0, Affine::default(), &self.bounds);
        self.header.paint(scene);
        self.content.paint(scene);
        scene.pop_layer();
    }

    fn layout(&mut self, size_hint: Size, font_cx: &mut FontContext) -> Size {
        self.size = size_hint;
        self.bounds = Rect::from_origin_size(
            Point::new(SHADOW_WIDTH, SHADOW_WIDTH),
            size_hint - Size::new(SHADOW_WIDTH * 2., SHADOW_WIDTH * 2.),
        )
        .to_rounded_rect(CORNER_RADIUS);
        let header_size = Size::new(self.bounds.width(), HEADER_BAR_HEIGHT);
        let content_size = Size::new(
            self.bounds.width(),
            self.bounds.height() - HEADER_BAR_HEIGHT,
        );

        self.header.layout(header_size, font_cx);

        self.content.layout(content_size, font_cx);
        self.header.size = header_size;
        self.header.position = Point::new(SHADOW_WIDTH, SHADOW_WIDTH);
        self.content.size = content_size;
        self.content.position = Point::new(SHADOW_WIDTH, SHADOW_WIDTH + HEADER_BAR_HEIGHT);
        size_hint
    }
    fn event(
        &mut self,
        event: event::WidgetEvent,
        event_cx: &mut event::EventCx<Message>,
    ) -> event::Status {
        // TODO: Should we always send this message?
        event_cx.push_internal_message(InternalMessage::TitleChanged(self.title.clone()));
        if let WidgetEvent::Mouse(event::mouse::Event::Press { position, button }) = event {
            if button == MouseButton::Left && !self.bounds.contains(position) {
                let x = (position.x / (self.size.width / 3.)) as u8;
                let y = (position.y / (self.size.height / 3.)) as u8;
                if x == 0 && y == 0 {
                    event_cx.push_internal_message(InternalMessage::DragResizeWindow(
                        ResizeDirection::NorthWest,
                    ));
                } else if x == 1 && y == 0 {
                    event_cx.push_internal_message(InternalMessage::DragResizeWindow(
                        ResizeDirection::North,
                    ));
                } else if x == 2 && y == 0 {
                    event_cx.push_internal_message(InternalMessage::DragResizeWindow(
                        ResizeDirection::NorthEast,
                    ));
                } else if x == 2 && y == 1 {
                    event_cx.push_internal_message(InternalMessage::DragResizeWindow(
                        ResizeDirection::East,
                    ));
                } else if x == 2 && y == 2 {
                    event_cx.push_internal_message(InternalMessage::DragResizeWindow(
                        ResizeDirection::SouthEast,
                    ));
                } else if x == 1 && y == 2 {
                    event_cx.push_internal_message(InternalMessage::DragResizeWindow(
                        ResizeDirection::South,
                    ));
                } else if x == 0 && y == 2 {
                    event_cx.push_internal_message(InternalMessage::DragResizeWindow(
                        ResizeDirection::SouthWest,
                    ));
                } else if x == 0 && y == 1 {
                    event_cx.push_internal_message(InternalMessage::DragResizeWindow(
                        ResizeDirection::West,
                    ));
                }
                return event::Status::Captured;
            }
        }

        event::Status::Ignored
    }
    fn children(&self) -> Vec<&WidgetData<Message>> {
        vec![&self.header, &self.content]
    }
    fn children_mut(&mut self) -> Vec<&mut WidgetData<Message>> {
        vec![&mut self.header, &mut self.content]
    }
    fn set_hover(&mut self, _hover: bool) -> event::Status {
        event::Status::Ignored
    }
}
