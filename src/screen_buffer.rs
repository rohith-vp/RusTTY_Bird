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
