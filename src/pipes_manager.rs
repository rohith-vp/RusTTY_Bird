use crate::{
    pipe::Pipe,
    screen_buffer::ScreenBuffer
};


pub struct PipesManager {
    screen_cols: i16,
    screen_rows: i16,
    pub pipes_vec: Vec<Pipe>
}

impl PipesManager {
    pub fn new(screen_cols: i16, screen_rows: i16) -> Self {
        let mut pipes_vec: Vec<Pipe> = Vec::new();
        pipes_vec.push(Pipe::new(screen_cols - Pipe::PIPE_WIDTH, screen_cols, screen_rows));
        Self { screen_cols, screen_rows, pipes_vec }
    }

    fn spawn_new_pipe(&mut self) {
        self.pipes_vec.push(Pipe::new(self.screen_cols + Pipe::PIPE_WIDTH + 1, self.screen_cols, self.screen_rows));
    }

    pub fn update(&mut self) {
        for pipe in self.pipes_vec.iter_mut() {
            pipe.update();
        }

        if self.pipes_vec[0].is_off_screen() {
            self.pipes_vec.remove(0);
        }

        if self.pipes_vec[self.pipes_vec.len() - 1].should_spawn_next() {
            self.spawn_new_pipe();
        }
    }

    pub fn draw(&self, buffer: &mut ScreenBuffer) {
        for pipe in self.pipes_vec.iter() {
            pipe.draw(buffer);
        }
    }
}
