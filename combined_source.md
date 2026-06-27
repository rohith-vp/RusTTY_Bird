# Project Source Code (Combined)
Generated on: Friday 26 June 2026 09:47:21 PM IST

## File: `src/bird.rs`

```rust
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

```

## File: `src/cell.rs`

```rust
use crossterm::{style::{Color::{self}}};


#[derive(Clone, Copy, PartialEq)]
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: Color::Reset,
            bg: Color::Reset
        }
    }
}

```

## File: `src/game.rs`

```rust
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

```

## File: `src/main.rs`

```rust
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
mod screen_buffer;
mod terminal_guard;
mod scoring_system;
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

    while running {
        let frame_start = Instant::now();

        match game.run() {
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

```

## File: `src/pipe.rs`

```rust
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
    cached_colors: [Color; Self::PIPE_WIDTH as usize],
}

impl Pipe {
    pub const PIPE_WIDTH: i16 = 5;
    pub const GAP_HEIGHT: i16 = 10;
    pub const PIPE_GAP: i16 = 15;

    pub const PIPE_COLOR_LEFTMOST: Color = Color::Rgb {
        r: 158,
        g: 227,
        b: 85,
    };
    pub const PIPE_COLOR_DARKEN_FACTOR: f32 = 0.82;

    pub fn new(x_int: i16, screen_cols: i16, screen_rows: i16) -> Self {
        let mut rng = rand::rng();
        let gap_middle = rng.random_range(Self::GAP_HEIGHT..(screen_rows - Self::GAP_HEIGHT));
        let x = x_int as f32;

        // Precompute the horizontal gradient profile *once* during instantiation
        let mut cached_colors = [Self::PIPE_COLOR_LEFTMOST; Self::PIPE_WIDTH as usize];
        let mut current_color = Self::PIPE_COLOR_LEFTMOST;
        for x_offset in 0..Self::PIPE_WIDTH as usize {
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
            cached_colors,
        }
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
        obs_x >= self.x_int
            && obs_x <= self.x_int + Self::PIPE_WIDTH
            && (obs_y <= self.gap_middle - Self::GAP_HEIGHT / 2
                || obs_y >= self.gap_middle + Self::GAP_HEIGHT / 2)
    }

    pub fn update(&mut self) {
        self.x -= 0.5;
        self.x_int = self.x.round() as i16;
    }

    pub fn draw(&self, buffer: &mut ScreenBuffer) {
        // Edge Case / Safety Check: If the pipe is entirely off-screen to the left,
        // skip rendering entirely. (Handled at PipesManager level, but ideal for safety).
        if self.x_int + Self::PIPE_WIDTH < 0 {
            return;
        }

        let gap_top = self.gap_middle - Self::GAP_HEIGHT / 2;
        let gap_bottom = self.gap_middle + Self::GAP_HEIGHT / 2;

        // Performance Optimization: Cache buffer width to minimize field lookups
        // Calculate bounded horizontal indices to prevent drawing off-screen cells
        let start_offset = if self.x_int < 0 { -self.x_int } else { 0 };
        let end_offset = Self::PIPE_WIDTH;

        for y in 0..self.screen_rows {
            // Bypass processing inside the gap region completely
            if y >= gap_top && y < gap_bottom {
                continue;
            }

            for x_offset in start_offset..end_offset {
                let world_x = self.x_int + x_offset;

                // Early break if the pipe columns cross past the right screen border
                if world_x >= self.screen_cols {
                    break;
                }

                let color = self.cached_colors[x_offset as usize];
                buffer.set(world_x, y, '█', color, color);
            }
        }
    }
}

```

## File: `src/pipes_manager.rs`

```rust
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
        pipes_vec.push_back(Pipe::new(screen_cols - Pipe::PIPE_WIDTH, screen_cols, screen_rows));
        Self { screen_cols, screen_rows, pipes_vec }
    }

    fn spawn_new_pipe(&mut self) {
        self.pipes_vec.push_back(Pipe::new(
            self.screen_cols + Pipe::PIPE_WIDTH + 1,
            self.screen_cols,
            self.screen_rows
        ));
    }

    pub fn update(&mut self) {
        for pipe in self.pipes_vec.iter_mut() {
            pipe.update();
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

```

## File: `src/scoring_system.rs`

```rust
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

```

## File: `src/screen_buffer.rs`

```rust
use std::{io::{Write, stdout}};

use crossterm::{
    QueueableCommand,
    style::{
        Color::{self},
        ResetColor,
        SetBackgroundColor,
        SetForegroundColor
    }
};

use crate::{
    cell::Cell,
    utils::darken_color
};


pub struct ScreenBuffer {
    width: u16,
    height: u16,
    current: Vec<Cell>,
    next: Vec<Cell>
}

impl ScreenBuffer {
    const BG_COLOR: Color = Color::Rgb { r: 135, g: 206, b: 250 };

    pub fn new(width: u16, height: u16) -> Self {
        let size = (width as usize) * (height as usize);
        Self {
            width,
            height,
            current: vec![Cell::default(); size],
            next: vec![Cell::default(); size]
        }
    }

    pub fn get_bg_color_at(&self, x: i16, y: i16) -> Color {
        // Safe Bounds Check: Verify coordinates are within the current screen boundaries
        if x >= 0 && x < self.width as i16 && y >= 0 && y < self.height as i16 {
            let idx = (y as usize) * (self.width as usize) + (x as usize);
            self.next[idx].bg
        } else {
            // Fail-safe default if the bird is clipping past the ceiling or floor
            Self::BG_COLOR
        }
    }

    pub fn clear_next(&mut self) {
        // for cell in self.next.iter_mut() {
        //     *cell = Cell::default();
        // }
        // for cell in self.next.iter_mut() {
        //     *cell = Cell {
        //         ch: ' ',
        //         fg: Color::Reset,
        //         bg: Self::BG_COLOR,
        //     };
        // }
        for y in 0..self.height {
            let factor = 1.0 - (y as f32 / self.height as f32) * 0.3;
            let row_bg_color = darken_color(Self::BG_COLOR, factor);

            for x in 0..self.width {
                let idx = (y as usize) * (self.width as usize) + (x as usize);
                self.next[idx] = Cell {
                    ch: ' ',
                    fg: Color::Reset,
                    bg: row_bg_color,
                };
            }
        }
    }

    pub fn set(&mut self, x: i16, y: i16, ch: char, fg: Color, bg: Color) {
        if x >= 0 && x < self.width as i16 && y >= 0 && y < self.height as i16 {
            let idx = (y as usize) * (self.width as usize) + (x as usize);
            self.next[idx] = Cell { ch, fg, bg };
        }
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        let mut out = stdout();
        let mut last_bg = Color::Reset;
        let mut last_fg = Color::Reset;

        out.queue(ResetColor)?;

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y as usize) * (self.width as usize) + (x as usize);
                let next_cell = self.next[idx];
                let current_cell = self.current[idx];

                if next_cell != current_cell {
                    out.queue(crossterm::cursor::MoveTo(x, y))?;

                    if next_cell.bg != last_bg {
                        out.queue(SetBackgroundColor(next_cell.bg))?;
                        last_bg = next_cell.bg;
                    }
                    if next_cell.fg != last_fg {
                        out.queue(SetForegroundColor(next_cell.fg))?;
                        last_fg = next_cell.fg;
                    }

                    print!("{}", next_cell.ch);
                    self.current[idx] = next_cell;
                }
            }
        }

        out.flush()?;
        Ok(())
    }
}

```

## File: `src/terminal_guard.rs`

```rust
use std::{io::{stdout}};

use crossterm::{
    cursor::{
        Hide,
        Show
    },
    execute,
    style::{
        ResetColor
    }, terminal::{
        EnterAlternateScreen,
        LeaveAlternateScreen,
        disable_raw_mode,
        enable_raw_mode
    }
};


pub struct TerminalGuard;

impl TerminalGuard {
    pub fn init() -> Self {
        enable_raw_mode().expect("Failed to enable raw mode.");
        execute!(stdout(), EnterAlternateScreen, Hide).expect("Failed to setup alternate screen.");
        TerminalGuard
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = execute!(stdout(), ResetColor, Show, LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}

```

## File: `src/utils.rs`

```rust
use crossterm::style::Color;


/// Darkens a crossterm Color.
/// `factor` should be between 0.0 (completely black) and 1.0 (no change).
pub fn darken_color(color: Color, factor: f32) -> Color {
    // Clamp the factor to guarantee safety and avoid unexpected overflow artifacts
    let factor = factor.clamp(0.0, 1.0);

    match color {
        // If it's an RGB color, mathematically scale the channels down
        Color::Rgb { r, g, b } => Color::Rgb {
            r: (r as f32 * factor).round() as u8,
            g: (g as f32 * factor).round() as u8,
            b: (b as f32 * factor).round() as u8,
        },

        // If it's a standard ANSI color, manually map it to its darker variant
        Color::AnsiValue(val) => Color::AnsiValue(darken_ansi_value(val)),

        // Fallback or explicit mapping for named enum variants
        Color::Blue => Color::DarkBlue,
        Color::Green => Color::DarkGreen,
        Color::Cyan => Color::DarkCyan,
        Color::Magenta => Color::DarkMagenta,
        Color::Red => Color::DarkRed,
        Color::Yellow => Color::AnsiValue(3), // ANSI 3 is Olive/Dark Yellow

        // If it's already dark, grayscale, or Reset, leave it as-is
        other => other,
    }
}


// Optional helper if your project relies on raw 8-bit ANSI colors (0-255)
fn darken_ansi_value(val: u8) -> u8 {
    // Simplistic safe fallback: if it's in the standard 16-color block, shift it down
    if val >= 8 && val <= 15 { val - 8 } else { val }
}

```

