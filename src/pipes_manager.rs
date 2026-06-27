use std::collections::VecDeque;
use crate::{
    pipe::Pipe,
    screen_buffer::ScreenBuffer
};


pub struct PipesManager {
    screen_cols: i16,
    screen_rows: i16,
    pub pipes_vec: VecDeque<Pipe>
}

impl PipesManager {
    pub fn new(screen_cols: i16, screen_rows: i16) -> Self {
        let mut pipes_vec = VecDeque::with_capacity(4);
        let initial_pipe = Pipe::new(screen_cols, screen_cols, screen_rows);
        let spawn_x = screen_cols - initial_pipe.width;
        pipes_vec.push_back(Pipe::new(spawn_x, screen_cols, screen_rows));
        Self { screen_cols, screen_rows, pipes_vec }
    }

    fn spawn_new_pipe(&mut self) {
        if let Some(last_pipe) = self.pipes_vec.back() {
            // Spawn perfectly behind the last pipe using its dynamic layout definitions
            let next_x = last_pipe.x_int + last_pipe.width + last_pipe.pipe_gap;
            self.pipes_vec.push_back(Pipe::new(
                next_x,
                self.screen_cols,
                self.screen_rows
            ));
        }
    }

    pub fn update(&mut self, dt: f32) {
        for pipe in self.pipes_vec.iter_mut() {
            pipe.update(dt);
        }

        if let Some(front_pipe) = self.pipes_vec.front() {
            if front_pipe.is_off_screen() {
                self.pipes_vec.pop_front();
            }
        }

        if let Some(last_pipe) = self.pipes_vec.back() {
            if last_pipe.should_spawn_next() {
                self.spawn_new_pipe();
            }
        }
    }

    pub fn draw(&self, buffer: &mut ScreenBuffer) {
        for pipe in self.pipes_vec.iter() {
            pipe.draw(buffer);
        }
    }
}
