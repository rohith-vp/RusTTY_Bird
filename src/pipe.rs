use crossterm::style::Color;
use rand::Rng;

use crate::{screen_buffer::ScreenBuffer, utils::darken_color};

pub struct Pipe {
    x: f32,
    pub x_int: i16,
    gap_middle: i16,
    screen_cols: i16,
    screen_rows: i16,
    pub scored: bool,

    // Dynamic scaling parameters calculated per resolution
    pub width: i16,
    pub gap_height: i16,
    pub pipe_gap: i16,
    cached_colors: Vec<Color>, // Changed from fixed array to Vec
}

impl Pipe {
    pub const PIPE_WIDTH_PERCENT: i16 = 4;      // 4% of screen width
    pub const GAP_HEIGHT_PERCENT: i16 = 30;     // 25% of screen height
    pub const PIPE_GAP_PERCENT: i16 = 25;       // 20% of screen width

    pub const PIPE_COLOR_LEFTMOST: Color = Color::Rgb {
        r: 158,
        g: 227,
        b: 85,
    };
    pub const PIPE_COLOR_DARKEN_FACTOR: f32 = 0.82;

    pub fn new(x_int: i16, screen_cols: i16, screen_rows: i16) -> Self {
        let mut rng = rand::rng();

        // Dynamic Scaling Calculations (Tweak percentages to perfect your balance)
        let width = (screen_cols * Self::PIPE_WIDTH_PERCENT / 100).max(3);
        let gap_height = (screen_rows * Self::GAP_HEIGHT_PERCENT / 100).max(6);
        let pipe_gap = (screen_cols * Self::PIPE_GAP_PERCENT / 100).max(10);

        let min_gap_y = gap_height;
        let max_gap_y = screen_rows - gap_height;
        let gap_middle = if min_gap_y >= max_gap_y {
            screen_rows / 2
        } else {
            rng.random_range(min_gap_y..max_gap_y)
        };

        let x = x_int as f32;

        // Populate your cached colors based on dynamic width
        let mut cached_colors = vec![Self::PIPE_COLOR_LEFTMOST; width as usize];
        let mut current_color = Self::PIPE_COLOR_LEFTMOST;
        for x_offset in 0..width as usize {
            cached_colors[x_offset] = current_color;
            current_color = darken_color(current_color, Self::PIPE_COLOR_DARKEN_FACTOR);
        }

        Self {
            x,
            x_int,
            gap_middle,
            screen_cols,
            screen_rows,
            scored: false,
            width,
            gap_height,
            pipe_gap,
            cached_colors,
        }
    }

    #[inline]
    pub fn is_off_screen(&self) -> bool {
        self.x_int + self.width < 0
    }

    #[inline]
    pub fn should_spawn_next(&self) -> bool {
        self.x_int + self.width + self.gap_height < self.screen_cols
    }

    pub fn check_collission(&self, obs_x: i16, obs_y: i16) -> bool {
        obs_x >= self.x_int
            && obs_x <= self.x_int + self.width
            && (obs_y <= self.gap_middle - self.gap_height / 2
                || obs_y >= self.gap_middle + self.gap_height / 2)
    }

    pub fn update(&mut self, dt: f32) {
        let frame_scale = dt * 60.0;
        let speed = (self.screen_cols as f32 * 0.005).max(0.3);
        self.x -= speed * frame_scale;
        self.x_int = self.x.round() as i16;
    }

    pub fn draw(&self, buffer: &mut ScreenBuffer) {
        if self.is_off_screen() { return; }

        let gap_top = self.gap_middle - self.gap_height / 2;
        let gap_bottom = self.gap_middle + self.gap_height / 2;

        let start_offset = if self.x_int < 0 { -self.x_int } else { 0 };
        let end_offset = self.width;

        for y in 0..self.screen_rows {
            if y >= gap_top && y < gap_bottom { continue; }

            for x_offset in start_offset..end_offset {
                let world_x = self.x_int + x_offset;
                if world_x >= self.screen_cols { break; }

                let color = self.cached_colors[x_offset as usize];
                buffer.set(world_x, y, '█', color, color);
            }
        }
    }
}
