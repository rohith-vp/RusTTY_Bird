use crossterm::style::Color;
use crate::screen_buffer::ScreenBuffer;


pub struct Bird {
    pub bird_y: f32,
    pub bird_y_vel: f32,
    screen_rows: i16
}

impl Bird {
    pub const BIRD_X: i16 = 30;
    const GRAVITY: f32 = 0.020;
    const JUMP_FORCE: f32 = -0.35;       // Upward burst (negative Y is UP)
    const TERMINAL_VELOCITY: f32 = 0.4;  // Max falling speed to prevent jerky skips
    pub const SPRITE_CELL_WIDTH: i16 = 3;
    // Game color palette definitions
    const COLOR_BODY: Color = Color::Rgb { r: 255, g: 215, b: 0 };   // Gold/Yellow
    const COLOR_WING: Color = Color::Rgb { r: 255, g: 69, b: 0 };    // Red-Orange
    const COLOR_BEAK: Color = Color::Rgb { r: 255, g: 140, b: 0 };   // Dark Orange

    pub fn new(screen_rows: i16) -> Self {
        let bird_y = screen_rows as f32 / 2.0;
        let bird_y_vel = 0.0;
        Bird { bird_y, bird_y_vel, screen_rows }
    }

    pub fn update(&mut self) {
        self.bird_y += self.bird_y_vel;
        self.bird_y_vel += Self::GRAVITY;

        if self.bird_y_vel > Self::TERMINAL_VELOCITY {
            self.bird_y_vel = Self::TERMINAL_VELOCITY;
        }

        let max_row = (self.screen_rows - 1) as f32;
        if self.bird_y > max_row {
            self.bird_y = max_row;
            self.bird_y_vel = 0.0; // Stop falling when hitting the ground
        }

        if self.bird_y < 0.0 {
            self.bird_y = 0.0;
            self.bird_y_vel = 0.0; // Stop upward momentum if hitting the ceiling
        }
    }

    pub fn jump(&mut self) {
        self.bird_y_vel = Self::JUMP_FORCE;
    }

    pub fn draw(&self, buffer: &mut ScreenBuffer) {
        let base_x = Self::BIRD_X;
        let base_y = self.bird_y as i16;
        let sky_color = buffer.get_bg_color_at(Bird::BIRD_X, base_y);

        // Top half is sky, bottom half is yellow body
        buffer.set(base_x, base_y, '▄', Self::COLOR_BODY, sky_color);
        // Top half is orange wing, bottom half is yellow body
        buffer.set(base_x + 1, base_y, '▄', Self::COLOR_BODY, Self::COLOR_WING);
        // Top half is orange beak, bottom half is sky background
        buffer.set(base_x + 2, base_y, '▀', Self::COLOR_BEAK, sky_color);
    }
}
