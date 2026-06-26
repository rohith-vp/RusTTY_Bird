use crossterm::style::Color;
use rand::Rng;

use crate::{
    screen_buffer::ScreenBuffer,
    utils::darken_color
};

pub struct Pipe {
    x: f32,
    pub x_int: i16,
    gap_middle: i16,
    screen_cols: i16,
    screen_rows: i16,
    pub scored: bool
}

impl Pipe {
    pub const PIPE_WIDTH: i16 = 5;
    pub const GAP_HEIGHT: i16 = 10;
    pub const PIPE_GAP: i16 = 15;

    pub const PIPE_COLOR_LEFTMOST: Color = Color::Rgb { r: 158, g: 227, b: 85 };
    pub const PIPE_COLOR_DARKEN_FACTOR: f32 = 0.82;

    pub fn new(x_int: i16, screen_cols: i16, screen_rows: i16) -> Self {
        let mut rng = rand::rng();
        let gap_middle = rng.random_range(Self::GAP_HEIGHT..(screen_rows - Self::GAP_HEIGHT));
        let x = x_int as f32;
        Self { x, x_int, gap_middle, screen_cols, screen_rows, scored: false }
    }

    #[inline]
    pub fn is_off_screen(&self) -> bool {
        self.x_int + Self::PIPE_WIDTH < 0
    }

    #[inline]
    pub fn should_spawn_next(&self) -> bool {
        self.x_int + Self::PIPE_WIDTH + Self::PIPE_GAP < self.screen_cols
    }

    pub fn check_collission(&self, obs_x: i16, obs_y: i16) -> bool {
        obs_x >= self.x_int &&
        obs_x <= self.x_int + Self::PIPE_WIDTH &&
        (
            obs_y <= self.gap_middle - Self::GAP_HEIGHT / 2 ||
            obs_y >= self.gap_middle + Self::GAP_HEIGHT / 2
        )
    }

    pub fn update(&mut self) {
        self.x -= 0.5;
        self.x_int = self.x.round() as i16;
    }

    pub fn draw(&self, buffer: &mut ScreenBuffer) {
        let mut row_colors = [Self::PIPE_COLOR_LEFTMOST; Self::PIPE_WIDTH as usize];
        let mid_point = (Self::PIPE_WIDTH / 2) as usize;

        let mut current_color = Self::PIPE_COLOR_LEFTMOST;

        for x_offset in 0..Self::PIPE_WIDTH as usize {
            row_colors[x_offset] = current_color;
            if x_offset + 1 == mid_point || x_offset + 1 > mid_point {
                current_color = darken_color(current_color, Self::PIPE_COLOR_DARKEN_FACTOR);
            } else {
                current_color = darken_color(current_color, Self::PIPE_COLOR_DARKEN_FACTOR);
            }
        }

        let gap_top = self.gap_middle - Self::GAP_HEIGHT / 2;
        let gap_bottom = self.gap_middle + Self::GAP_HEIGHT / 2;

        for y in 0..self.screen_rows {
            if y >= gap_top && y < gap_bottom {
                continue;
            }
            for (x_offset, &color) in row_colors.iter().enumerate() {
                buffer.set(self.x_int + x_offset as i16, y, '█', color, color);
            }
        }
    }
}
