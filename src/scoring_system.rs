use crossterm::style::Color;

use crate::{
    screen_buffer::ScreenBuffer
};


pub struct ScoringSystem {
    pub score: u32
}

impl ScoringSystem {
    const FG_COLOR: Color = Color::White;
    const BG_COLOR: Color = Color::Black;

    pub fn new() -> Self {
        let score: u32 = 0;
        Self { score }
    }

    pub fn increment_score(&mut self) {
        self.score += 1;
    }

    fn draw_text(start_x: i16, y: i16, text: &str, fg: Color, bg: Color, buffer: &mut ScreenBuffer) {
        for (i, ch) in text.chars().enumerate() {
            buffer.set(start_x + i as i16, y, ch, fg, bg);
        }
    }

    pub fn draw_score_board(&self, buffer: &mut ScreenBuffer) -> std::io::Result<bool> {
        let score_string = format!(" SCORE: {} ", self.score);
        Self::draw_text(1, 1, &score_string, Self::FG_COLOR, Self::BG_COLOR, buffer);
        Ok(true)
    }
}
