#![allow(dead_code)]
#![allow(unused_imports)]

mod tasks;
mod course;
mod ui;
mod app;

// Probably bad practice but w/e it works for now
use course::*;
use ui::*;
use app::*;

use std::{io, fs, process, os::unix::prelude::PermissionsExt};
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


fn close_application() -> io::Result<()> {
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

fn run_app<B>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()>
    where B: Backend, {

    loop {
        terminal.draw(|f| {
            ui(f, app)
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('j') => app.next(),
                KeyCode::Char('k') => app.previous(),
                KeyCode::Enter => app.update_highlighted_course(),
                _ => {},
            }
        }
    }
}


fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout,
        EnterAlternateScreen,
        EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    // Create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // Clean up the terminal again
    close_application()?;
    execute!(
        terminal.backend_mut(),
    )?;
    terminal.show_cursor()?;

    match res {
        Ok(_) => { },
        Err(res) => {
            println!("{:?}", res);
        }
    }

    Ok(())
}

