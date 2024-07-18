// Copyright 2019 Héctor Ramón, Iced contributors
use std::time::{Duration, Instant};
use tracing::{info, span, Level};
pub struct DebugLayer {
    startup_start: Instant,
    startup_duration: Duration,

    update_start: Instant,
    update_durations: TimeBuffer,

    view_start: Instant,
    view_durations: TimeBuffer,

    layout_start: Instant,
    layout_durations: TimeBuffer,

    event_start: Instant,
    event_durations: TimeBuffer,

    draw_start: Instant,
    draw_durations: TimeBuffer,

    encode_start: Instant,
    encode_durations: TimeBuffer,

    render_start: Instant,
    render_durations: TimeBuffer,
}

impl DebugLayer {
    /// Creates a new [`struct@Debug`].
    pub fn new() -> Self {
        let now = Instant::now();

        Self {
            startup_start: now,
            startup_duration: Duration::from_secs(0),

            update_start: now,
            update_durations: TimeBuffer::new(10),

            view_start: now,
            view_durations: TimeBuffer::new(10),

            layout_start: now,
            layout_durations: TimeBuffer::new(10),

            event_start: now,
            event_durations: TimeBuffer::new(10),

            draw_start: now,
            draw_durations: TimeBuffer::new(10),
            encode_start: now,
            encode_durations: TimeBuffer::new(10),

            render_start: now,
            render_durations: TimeBuffer::new(10),
        }
    }

    pub fn startup_started(&mut self) {
        self.startup_start = Instant::now();
    }

    pub fn startup_finished(&mut self) {
        self.startup_duration = self.startup_start.elapsed();
    }

    pub fn update_started(&mut self) {
        self.update_start = Instant::now();
    }

    pub fn update_finished(&mut self) {
        self.update_durations.push(self.update_start.elapsed());
    }

    pub fn view_started(&mut self) {
        self.view_start = Instant::now();
    }

    pub fn view_finished(&mut self) {
        self.view_durations.push(self.view_start.elapsed());
    }

    pub fn layout_started(&mut self) {
        self.layout_start = Instant::now();
    }

    pub fn layout_finished(&mut self) {
        self.layout_durations.push(self.layout_start.elapsed());
    }

    pub fn event_started(&mut self) {
        self.event_start = Instant::now();
    }

    pub fn event_finished(&mut self) {
        self.event_durations.push(self.event_start.elapsed());
    }

    pub fn draw_started(&mut self) {
        self.draw_start = Instant::now();
    }

    pub fn draw_finished(&mut self) {
        self.draw_durations.push(self.draw_start.elapsed());
    }

    pub fn encode_started(&mut self) {
        self.encode_start = Instant::now();
    }

    pub fn encode_finished(&mut self) {
        self.encode_durations.push(self.encode_start.elapsed());
    }

    pub fn render_started(&mut self) {
        self.render_start = Instant::now();
    }

    pub fn render_finished(&mut self) {
        self.render_durations.push(self.render_start.elapsed());
    }

    pub fn log(&mut self) {
        let span = span!(Level::INFO, "perf");
        let _guard = span.enter();
        info!("Startup: duration={:?}", self.startup_duration);
        info!(
            "Update: avg={:?}, max={:?}",
            self.update_durations.average(),
            self.update_durations.max()
        );
        info!(
            "View: avg={:?}, max={:?}",
            self.view_durations.average(),
            self.view_durations.max()
        );
        info!(
            "Layout: avg={:?}, max={:?}",
            self.layout_durations.average(),
            self.layout_durations.max()
        );
        info!(
            "Event: avg={:?}, max={:?}",
            self.event_durations.average(),
            self.event_durations.max()
        );
        info!(
            "Draw: avg={:?}, max={:?}",
            self.draw_durations.average(),
            self.draw_durations.max()
        );
        info!(
            "Encode: avg={:?}, max={:?}",
            self.encode_durations.average(),
            self.encode_durations.max()
        );
        info!(
            "Render: avg={:?}, max={:?}",
            self.render_durations.average(),
            self.render_durations.max()
        );
    }
}
#[derive(Debug)]
struct TimeBuffer {
    head: usize,
    size: usize,
    contents: Vec<Duration>,
}

impl TimeBuffer {
    fn new(capacity: usize) -> TimeBuffer {
        TimeBuffer {
            head: 0,
            size: 0,
            contents: vec![Duration::from_secs(0); capacity],
        }
    }

    fn push(&mut self, duration: Duration) {
        self.head = (self.head + 1) % self.contents.len();
        self.contents[self.head] = duration;
        self.size = (self.size + 1).min(self.contents.len());
    }

    fn average(&self) -> Duration {
        let sum: Duration = if self.size == self.contents.len() {
            self.contents[..].iter().sum()
        } else {
            self.contents[..self.size].iter().sum()
        };

        sum / self.size.max(1) as u32
    }
    fn max(&self) -> Duration {
        *self
            .contents
            .iter()
            .max()
            .unwrap_or(&Duration::from_secs(0))
    }
}
