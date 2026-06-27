use std::{
    io::stdout,
    panic,
    time::{Duration, Instant},
};

use crossterm::{
    cursor::Show,
    execute,
    style::ResetColor,
    terminal::{LeaveAlternateScreen, disable_raw_mode},
};

mod bird;
mod cell;
mod game;
mod pipe;
mod pipes_manager;
mod scoring_system;
mod screen_buffer;
mod terminal_guard;
mod utils;

use game::Game;
use terminal_guard::TerminalGuard;

fn main() -> std::io::Result<()> {
    let _guard = TerminalGuard::init();

    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        execute!(stdout(), ResetColor, Show, LeaveAlternateScreen).ok();
        let _ = disable_raw_mode();
        default_hook(panic_info);
    }));

    let (cols, rows) = crossterm::terminal::size()?;

    let mut running = true;
    let mut game = Game::new(cols as i16, rows as i16);

    let target_frame_time = Duration::from_millis(17); // 60 FPS
    let mut last_frame_time = Instant::now();

    while running {
        let frame_start = Instant::now();
        let dt = frame_start.duration_since(last_frame_time).as_secs_f32();
        last_frame_time = frame_start;

        match game.run(dt) {
            Ok(keep_running) => {
                running = keep_running;
            }
            Err(e) => {
                panic!("Terminal input error: {}", e);
            }
        }

        let time_elapsed = frame_start.elapsed();
        if time_elapsed < target_frame_time {
            std::thread::sleep(target_frame_time - time_elapsed);
        }
    }

    Ok(())
}
