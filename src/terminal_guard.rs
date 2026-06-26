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
