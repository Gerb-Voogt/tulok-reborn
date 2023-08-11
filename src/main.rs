#![allow(dead_code)]
#![allow(unused_imports)]

mod tasks;
mod course;
mod ui;

use course::*;
use ui::*;

use std::{io, fs, process};
use chrono;
use tui::{
    backend::{CrosstermBackend, Backend},
    widgets::{Widget, Block, Borders, TableState, Table, Cell, Row},
    layout::{Layout, Constraint, Direction, Rect},
    Terminal,
    Frame, text::Span, style::{Style, Color},
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{size, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use serde::{Deserialize, Serialize};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            ui(f)
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                _ => {},
            }
        }
    }

    // Clean up the terminal again
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
