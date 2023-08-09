use std::{io, fs, process};
use tui::{
    backend::{CrosstermBackend, Backend, self},
    widgets::{Widget, Block, Borders, TableState, Table, Cell, Row},
    layout::{Layout, Constraint, Direction, Rect},
    Terminal,
    Frame,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{size, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use serde::{Deserialize, Serialize};


const COURSES_DIR: &str = "/home/gerb/uni/courses/";

// Deriving these allows for automagically importing the yaml files
#[derive(Serialize, Deserialize)] 
struct Course {
    title: String,
    short: String,
    code: String,
    year: String,
    quarter: String,
    url: String,
    active: bool,
}

/// Note that in the current state this function may panic
/// it would be better to rewrite this such that the error is handled explicitly
/// rather then the program crashing outright
fn create_course_from_yaml_file(course_path: &str) -> Course {
    // Read the file
    let f = std::fs::File::open(course_path.to_owned() + "/info.yaml").expect("Could not read file!");
    serde_yaml::from_reader(f).expect("Could not read file!")
}

fn retrieve_courses_active(courses_dir: &str) -> Vec<Course> {

}


fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(f.size());


    let chunk_left = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(chunks[0]);

    let chunk_right = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(chunks[1]);

    let course = create_course_from_yaml_file("/home/gerb/uni/courses/SC/SC42056-OPT/");

    draw_course_info_block(course, f, chunk_left[0]);


    let block = Block::default()
        .title("Current Courses")
        .borders(Borders::ALL);
    f.render_widget(block, chunk_left[1]);


    let block = Block::default()
        .title("Up Next")
        .borders(Borders::ALL);
    f.render_widget(block, chunk_right[0]);
}


fn draw_course_info_block<B>(course: Course, f: &mut Frame<B>, layout_chunk: Rect)
    where B: Backend {

    let t = Table::new(vec![
        Row::new(vec![Cell::from(format!("Code: {}", course.code))]),
        Row::new(vec![Cell::from(format!("Title: {}", course.title))]),
        Row::new(vec![Cell::from(format!("Year: {}", course.year))]),
        Row::new(vec![Cell::from(format!("Quarter: {}", course.quarter))]),
        Row::new(vec![Cell::from(format!("Active: {}", course.active.to_string()))]),
        ])
        .block(Block::default().borders(Borders::ALL).title("Course Info"))
        .widths(&[Constraint::Percentage(100)]);

    f.render_widget(t, layout_chunk);
}


fn draw_current_courses_block<B>(f: &mut Frame<B>, layout_chunk: Rect) 
    where B: Backend {

}


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
