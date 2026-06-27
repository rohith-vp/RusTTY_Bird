use crate::screen_buffer::ScreenBuffer;
use crossterm::style::Color;

pub struct Bird {
    pub bird_y: f32,
    pub bird_y_vel: f32,
    screen_cols: i16,
    screen_rows: i16,
    gravity: f32,
    jump_force: f32,
    terminal_velocity: f32,
}

impl Bird {
    const BIRD_X_REL_POS: i16 = 20;             // 20% from left of the screen
    const GRAVITY_REL: f32 = 0.0008;
    const JUMP_FORCE_REL: f32 = 0.012;
    const TERMINAL_VELOCITY_REL: f32 = 0.012;
    pub const SPRITE_CELL_WIDTH: i16 = 3;

    // Game color palette definitions
    const COLOR_BODY: Color = Color::Rgb {
        r: 255,
        g: 215,
        b: 0,
    }; // Gold/Yellow
    const COLOR_WING: Color = Color::Rgb {
        r: 255,
        g: 69,
        b: 0,
    }; // Red-Orange
    const COLOR_BEAK: Color = Color::Rgb {
        r: 255,
        g: 140,
        b: 0,
    }; // Dark Orange

    pub fn new(screen_cols: i16, screen_rows: i16) -> Self {
        let bird_y = screen_rows as f32 / 2.0;
        let bird_y_vel = 0.0;
        let gravity = screen_rows as f32 * Self::GRAVITY_REL;
        let jump_force = -(screen_rows as f32 * Self::JUMP_FORCE_REL);
        let terminal_velocity = screen_rows as f32 * Self::TERMINAL_VELOCITY_REL;
        Bird {
            bird_y,
            bird_y_vel,
            screen_cols,
            screen_rows,
            gravity,
            jump_force,
            terminal_velocity,
        }
    }

    #[inline]
    pub fn get_x(&self) -> i16 {
        self.screen_cols * Self::BIRD_X_REL_POS / 100
    }

    pub fn update(&mut self, dt: f32) {
        let frame_scale = dt * 60.0;

        self.bird_y += self.bird_y_vel * frame_scale;
        self.bird_y_vel += self.gravity * frame_scale;

        if self.bird_y_vel > self.terminal_velocity {
            self.bird_y_vel = self.terminal_velocity;
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
        self.bird_y_vel = self.jump_force;
    }

    pub fn draw(&self, buffer: &mut ScreenBuffer) {
        let base_x = self.get_x();
        let base_y = self.bird_y as i16;
        let sky_color = buffer.get_bg_color_at(base_x, base_y);

        // Top half is sky, bottom half is yellow body
        buffer.set(base_x, base_y, '▄', Self::COLOR_BODY, sky_color);
        // Top half is orange wing, bottom half is yellow body
        buffer.set(base_x + 1, base_y, '▄', Self::COLOR_BODY, Self::COLOR_WING);
        // Top half is orange beak, bottom half is sky background
        buffer.set(base_x + 2, base_y, '▀', Self::COLOR_BEAK, sky_color);
    }
}
