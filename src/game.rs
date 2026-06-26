use std::time::Duration;
use crossterm::event::{ self, Event, KeyCode, KeyEvent, KeyModifiers };

use crate::{
    bird::Bird,
    pipe::Pipe,
    pipes_manager::PipesManager,
    scoring_system::ScoringSystem,
    screen_buffer::ScreenBuffer
};


pub struct Game {
    buffer: ScreenBuffer,
    bird: Bird,
    pipes_manager: PipesManager,
    scoring_system: ScoringSystem
}

impl Game {
    pub fn new(screen_cols: i16, screen_rows: i16) -> Self {
        let buffer = ScreenBuffer::new(screen_cols as u16, screen_rows as u16);
        let bird = Bird::new(screen_rows);
        let pipes_manager = PipesManager::new(screen_cols, screen_rows);
        let scoring_system = ScoringSystem::new();
        Self { buffer, bird, pipes_manager, scoring_system }
    }

    pub fn has_bird_collided(&self) -> bool {
        for pipe in self.pipes_manager.pipes_vec.iter() {
            // Loop through all 3 horizontal cells that make up the bird's sprite width
            for offset in 0..crate::bird::Bird::SPRITE_CELL_WIDTH {
                let current_bird_x = crate::bird::Bird::BIRD_X + offset;
                let current_bird_y = self.bird.bird_y as i16;

                if pipe.check_collission(current_bird_x, current_bird_y) {
                    return true;
                }
            }
        }
        false
    }

    pub fn did_bird_passthrough(&mut self) -> bool {
        for pipe in self.pipes_manager.pipes_vec.iter_mut() {
            if !pipe.scored && (pipe.x_int + Pipe::PIPE_WIDTH) < Bird::BIRD_X {
                pipe.scored = true;
                return true;
            }
        }
        false
    }

    pub fn run(&mut self) -> std::io::Result<bool> {
        // Handle keyboard input
        while event::poll(Duration::from_millis(0))? {
            if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
                match code {
                    KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(false);
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        self.bird.jump();
                        break;
                    }
                    _ => {}
                }
            }
        }

        if self.has_bird_collided() {
            println!("Collission detected!");
            return Ok(false);
        }

        self.bird.update();
        self.pipes_manager.update();

        if self.did_bird_passthrough() {
            self.scoring_system.increment_score();
        }

        self.buffer.clear_next();

        self.bird.draw(&mut self.buffer);
        self.pipes_manager.draw(&mut self.buffer);
        self.scoring_system.draw_score_board(&mut self.buffer)?;

        self.buffer.flush()?;

        Ok(true)
    }
}
