use crate::widget::{Constraints, Length, Widget, WidgetSize};
use core::f64::consts::PI;
use parley::FontContext;
use ralaire_core::event::mouse::MouseButton;
use ralaire_core::event::{ResizeDirection, WidgetEvent};
use ralaire_core::{
    event, Affine, AppMessage, BlendMode, Color, InternalMessage, Point, Rect, RenderCx,
    RoundedRect, Shape, Size, WidgetId, WidgetIdPath,
};
use ralaire_core::{Brush, Circle, Gradient};

use super::WidgetData;

const CORNER_RADIUS: f64 = 12.;
const SHADOW_WIDTH: f64 = 15.;
const HEADER_BAR_HEIGHT: f64 = 32.;
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
pub struct RootWidget<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    pub id: WidgetId,
    bounds: RoundedRect,
    size: Size, // includes shadows
    content: WidgetData<Message>,
}

impl<Message> RootWidget<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    pub fn new(content: WidgetData<Message>) -> Self {
        RootWidget {
            id: WidgetId::unique(),
            bounds: Rect::ZERO.to_rounded_rect(CORNER_RADIUS),
            size: Size::ZERO,
            content,
        }
    }

    pub fn content(&mut self) -> &mut WidgetData<Message> {
        &mut self.content
    }
}
impl<Message> Widget<Message> for RootWidget<Message>
where
    Message: core::fmt::Debug + Clone + 'static,
{
    // Normally shadows are implemented with blur, but here we approximate the gaussian
    // function exp(-8x^2) by using 11 color points and linear interpolation between
    // the SHADOW_COLOR and SHADOW_FADE_COLOR where the factor is given by the function
    fn draw(&self, render_cx: &mut RenderCx) {
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
        render_cx.fill_shape(
            Affine::default(),
            &Rect::from_origin_size(
                Point::new(SHADOW_WIDTH + CORNER_RADIUS, 0.),
                Size::new(
                    self.size.width - (SHADOW_WIDTH + CORNER_RADIUS) * 2.,
                    SHADOW_WIDTH + CORNER_RADIUS,
                ),
            ),
            Brush::Gradient(
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
        );
        // bottom shadow
        render_cx.fill_shape(
            Affine::default(),
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
            Brush::Gradient(
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
        );
        // right shadow
        render_cx.fill_shape(
            Affine::default(),
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
            Brush::Gradient(
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
        );
        // left shadow
        render_cx.fill_shape(
            Affine::default(),
            &Rect::from_origin_size(
                Point::new(0., SHADOW_WIDTH + CORNER_RADIUS),
                Size::new(
                    SHADOW_WIDTH + CORNER_RADIUS,
                    self.size.height - (SHADOW_WIDTH + CORNER_RADIUS) * 2.,
                ),
            ),
            Brush::Gradient(
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
        );

        // corner shadows

        // top left
        render_cx.fill_shape(
            Affine::default(),
            &Circle::new(
                Point::new(SHADOW_WIDTH + CORNER_RADIUS, SHADOW_WIDTH + CORNER_RADIUS),
                SHADOW_WIDTH + CORNER_RADIUS,
            )
            .segment(0., PI, 1. / 2. * PI),
            Brush::Gradient(
                Gradient::new_radial(
                    Point::new(SHADOW_WIDTH + CORNER_RADIUS, SHADOW_WIDTH + CORNER_RADIUS),
                    (SHADOW_WIDTH + CORNER_RADIUS) as f32,
                )
                .with_stops(shadow_color_stops),
            ),
        );
        // top right
        render_cx.fill_shape(
            Affine::default(),
            &Circle::new(
                Point::new(
                    self.size.width - (SHADOW_WIDTH + CORNER_RADIUS),
                    SHADOW_WIDTH + CORNER_RADIUS,
                ),
                SHADOW_WIDTH + CORNER_RADIUS,
            )
            .segment(0., 3. / 2. * PI, 1. / 2. * PI),
            Brush::Gradient(
                Gradient::new_radial(
                    Point::new(
                        self.size.width - (SHADOW_WIDTH + CORNER_RADIUS),
                        SHADOW_WIDTH + CORNER_RADIUS,
                    ),
                    (SHADOW_WIDTH + CORNER_RADIUS) as f32,
                )
                .with_stops(shadow_color_stops),
            ),
        );
        // bottom right
        render_cx.fill_shape(
            Affine::default(),
            &Circle::new(
                Point::new(
                    self.size.width - (SHADOW_WIDTH + CORNER_RADIUS),
                    self.size.height - (SHADOW_WIDTH + CORNER_RADIUS),
                ),
                SHADOW_WIDTH + CORNER_RADIUS,
            )
            .segment(0., 0., 1. / 2. * PI),
            Brush::Gradient(
                Gradient::new_radial(
                    Point::new(
                        self.size.width - (SHADOW_WIDTH + CORNER_RADIUS),
                        self.size.height - (SHADOW_WIDTH + CORNER_RADIUS),
                    ),
                    (SHADOW_WIDTH + CORNER_RADIUS) as f32,
                )
                .with_stops(shadow_color_stops),
            ),
        );
        // bottom left
        render_cx.fill_shape(
            Affine::default(),
            &Circle::new(
                Point::new(
                    SHADOW_WIDTH + CORNER_RADIUS,
                    self.size.height - (SHADOW_WIDTH + CORNER_RADIUS),
                ),
                SHADOW_WIDTH + CORNER_RADIUS,
            )
            .segment(0., 1. / 2. * PI, 1. / 2. * PI),
            Brush::Gradient(
                Gradient::new_radial(
                    Point::new(
                        SHADOW_WIDTH + CORNER_RADIUS,
                        self.size.height - (SHADOW_WIDTH + CORNER_RADIUS),
                    ),
                    (SHADOW_WIDTH + CORNER_RADIUS) as f32,
                )
                .with_stops(shadow_color_stops),
            ),
        );

        render_cx.fill_shape(
            Affine::default(),
            &self.bounds,
            Color::parse("#fcfaf1").unwrap(),
        )
    }
    fn size_hint(&self) -> WidgetSize {
        WidgetSize {
            width: Length::Flexible(1),
            height: Length::Flexible(1),
        }
    }
    fn render(&self, render_cx: &mut RenderCx) {
        render_cx.push_layer(
            BlendMode::default(),
            Affine::default(),
            self.size.to_rounded_rect(0.),
        );
        self.draw(render_cx);
        render_cx.push_layer(BlendMode::default(), Affine::default(), self.bounds);
        let position = self.content.position;
        let size = self.content.size;
        let bounds = Rect::from_origin_size(position, size)
            .to_rounded_rect(self.content.widget.bounds_radii());
        render_cx.push_layer(BlendMode::default(), Affine::default(), bounds);
        self.content.widget.render(render_cx);
        render_cx.pop_layer();
        render_cx.pop_layer();
        render_cx.pop_layer();
    }
    fn layout(&mut self, constraints: Constraints, font_cx: &mut FontContext) {
        let size = constraints.max_size;
        self.size = size;
        self.bounds = Rect::from_origin_size(
            Point::new(SHADOW_WIDTH, SHADOW_WIDTH),
            size - Size::new(SHADOW_WIDTH * 2., SHADOW_WIDTH * 2.),
        )
        .to_rounded_rect(CORNER_RADIUS);
        let content_max_size = self.bounds.rect().size();

        self.content.widget.layout(
            Constraints {
                min_size: Size::ZERO,
                max_size: content_max_size,
            },
            font_cx,
        );

        let WidgetSize { width, height } = self.content.widget.size_hint();
        self.content.size = match (width, height) {
            (Length::Fixed(w), Length::Fixed(h)) => Size::new(w, h),
            (Length::Fixed(w), Length::Flexible(_)) => Size::new(w, content_max_size.height),
            (Length::Flexible(_), Length::Fixed(h)) => Size::new(content_max_size.width, h),
            (Length::Flexible(_), Length::Flexible(_)) => content_max_size,
        };
        self.content.position = Point::new(0., 0.);
    }
    fn event(
        &mut self,
        event: event::WidgetEvent,
        event_cx: &mut event::EventCx<AppMessage<Message>>,
    ) -> event::Status {
        match event {
            WidgetEvent::Mouse(mouse_event) => match mouse_event {
                event::mouse::Event::Press { position, button } => {
                    if button == MouseButton::Left {
                        if !self.bounds.contains(position) {
                            let x = (position.x / (self.size.width / 3.)) as u8;
                            let y = (position.y / (self.size.height / 3.)) as u8;
                            if x == 0 && y == 0 {
                                event_cx.add_message(AppMessage::Internal(
                                    InternalMessage::DragResizeWindow(ResizeDirection::NorthWest),
                                ));
                                event::Status::Captured
                            } else if x == 1 && y == 0 {
                                event_cx.add_message(AppMessage::Internal(
                                    InternalMessage::DragResizeWindow(ResizeDirection::North),
                                ));
                                event::Status::Captured
                            } else if x == 2 && y == 0 {
                                event_cx.add_message(AppMessage::Internal(
                                    InternalMessage::DragResizeWindow(ResizeDirection::NorthEast),
                                ));
                                event::Status::Captured
                            } else if x == 2 && y == 1 {
                                event_cx.add_message(AppMessage::Internal(
                                    InternalMessage::DragResizeWindow(ResizeDirection::East),
                                ));
                                event::Status::Captured
                            } else if x == 2 && y == 2 {
                                event_cx.add_message(AppMessage::Internal(
                                    InternalMessage::DragResizeWindow(ResizeDirection::SouthEast),
                                ));
                                event::Status::Captured
                            } else if x == 1 && y == 2 {
                                event_cx.add_message(AppMessage::Internal(
                                    InternalMessage::DragResizeWindow(ResizeDirection::South),
                                ));
                                event::Status::Captured
                            } else if x == 0 && y == 2 {
                                event_cx.add_message(AppMessage::Internal(
                                    InternalMessage::DragResizeWindow(ResizeDirection::SouthWest),
                                ));
                                event::Status::Captured
                            } else if x == 0 && y == 1 {
                                event_cx.add_message(AppMessage::Internal(
                                    InternalMessage::DragResizeWindow(ResizeDirection::West),
                                ));
                                event::Status::Captured
                            } else {
                                event::Status::Ignored
                            }
                        } else if Rect::from_origin_size(
                            Point::new(SHADOW_WIDTH, SHADOW_WIDTH),
                            Size::new(self.bounds.width(), HEADER_BAR_HEIGHT),
                        )
                        .to_rounded_rect((CORNER_RADIUS, CORNER_RADIUS, 0., 0.))
                        .contains(position)
                        {
                            // event_cx
                            //     .add_message(AppMessage::Internal(InternalMessage::DragMoveWindow));
                            // event::Status::Captured
                            event::Status::Ignored
                        } else {
                            event::Status::Ignored
                        }
                    } else {
                        event::Status::Ignored
                    }
                }
                _ => event::Status::Ignored,
            },
            _ => event::Status::Ignored,
        }
    }
    fn children(&self) -> Vec<&WidgetData<Message>> {
        vec![&self.content]
    }
    fn children_mut(&mut self) -> Vec<&mut WidgetData<Message>> {
        vec![&mut self.content]
    }
    fn bounds_tree(
        &self,
        _id_path: WidgetIdPath,
        _position: Point,
    ) -> Vec<(WidgetIdPath, RoundedRect)> {
        let position = Point::new(SHADOW_WIDTH, SHADOW_WIDTH);
        let id_path = vec![self.id];
        let mut v = vec![(id_path.clone(), self.bounds)];
        for child in self.children() {
            let mut child_id_path = id_path.clone();
            child_id_path.push(child.id);
            v.push((
                child_id_path.clone(),
                Rect::from_origin_size(position + child.position.to_vec2(), child.size)
                    .to_rounded_rect(child.widget.bounds_radii()),
            ));
            v.extend_from_slice(
                &child
                    .widget
                    .bounds_tree(child_id_path.clone(), position + child.position.to_vec2()),
            )
        }
        v
    }
}
