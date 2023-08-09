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
/// 
/// It might also be better to make it such that this function
/// accepts an actual path object rather then a string.
fn create_course_from_yaml_file(course_path: &str) -> Course {
    // Read the file
    let f = std::fs::File::open(course_path.to_owned() + "/info.yaml").expect("Could not read file!");
    serde_yaml::from_reader(f).expect("Could not read file!")
}

/// This function may panic. Should make it so explictly return a Result type
/// This function is also kinda gross. Need to figure out whether there is a better way to do this
/// because it has unwraps all over the place.
fn retrieve_courses_active(courses_dir: &str) -> Vec<Course> {
    let mut course_list: Vec<Course> = Vec::new();

    let base_course_code_paths = fs::read_dir(courses_dir).unwrap();
    for course_paths in base_course_code_paths {
        let course_dirs = fs::read_dir(course_paths.unwrap().path().display().to_string()).unwrap();
        for course_dir in course_dirs {
            let course = create_course_from_yaml_file(&course_dir.unwrap().path().display().to_string());
            if course.active == true {
                course_list.push(course);
            }
        }
    }
    course_list
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
        .constraints([Constraint::Percentage(25), Constraint::Percentage(25), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);

    let chunk_right = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(chunks[1]);

    let course = create_course_from_yaml_file("/home/gerb/uni/courses/SC/SC42150-SSP/");

    draw_course_info_block(&course, f, chunk_left[0]);
    draw_course_operations(f, chunk_left[1]);
    draw_current_courses_block(f, chunk_left[2]);

    draw_tasks_view(&course, f, chunk_right[0]);

    let block = Block::default()
        .title("Up Next")
        .borders(Borders::ALL);
    f.render_widget(block, chunk_right[0]);
}


fn draw_course_info_block<B>(course: &Course, f: &mut Frame<B>, layout_chunk: Rect)
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

    let mut rows: Vec<Row> = Vec::new();
    let active_courses = retrieve_courses_active(COURSES_DIR);

    for course in active_courses {
        rows.push(
            Row::new(vec![Cell::from(format!("{}-{}", course.code, course.short))]),
        )
    }
    let t = Table::new(rows)
        .block(Block::default().borders(Borders::ALL).title("Active Courses"))
        .widths(&[Constraint::Percentage(100)]);
    f.render_widget(t, layout_chunk);
}

fn draw_course_operations<B>(f: &mut Frame<B>, layout_chunk: Rect)
    where B: Backend {

    let t = Table::new(vec![
        Row::new(vec![Cell::from(format!("Open Files"))]),
        Row::new(vec![Cell::from(format!("Open Notes"))]),
        Row::new(vec![Cell::from(format!("Open Brightspace"))]),
        ])
        .block(Block::default().borders(Borders::ALL).title("Operations"))
        .widths(&[Constraint::Percentage(100)]);

    f.render_widget(t, layout_chunk);
}

fn draw_tasks_view<B>(course: &Course, f: &mut Frame<B>, layout_chunk: Rect)
    where B: Backend {

    let t = get_tasks_for_course(&course)
        .block(Block::default().borders(Borders::ALL).title("Up Next"))
        .widths(&[Constraint::Percentage(100)]);
    f.render_widget(t, layout_chunk);
}

/// TODO: Fix this to instead directly hook into the raw taskwarrior data rather then
/// using the roundabout way of accesing through the command that I am using now
fn get_tasks_for_course(course: &Course) -> Table<'static> {
    let header = process::Command::new("sh")
        .arg("-c")
        .arg(format!("task rc.verbose=none +{} | awk 'NR==2' | tr -s ' ' | tr ' ' ','", course.code))
        .output()
        .expect("Failed to run command");
    let output = process::Command::new("sh")
        .arg("-c")
        .arg(format!("task rc.verbose=none +{} | awk 'NR>=4' | tr -s ' ' | tr ' ', ','", course.code))
        .output()
        .expect("Failed to run command");

    let header_raw = String::from_utf8(header.stdout).unwrap();
    let header_tags: Vec<&str> = header_raw.split(",").collect();

    let output_raw = String::from_utf8(output.stdout).unwrap();
    let output_rows: Vec<&str> = output_raw.split('\n').collect();

    let mut rows: Vec<Row> = Vec::new();
    for row in output_rows {
        // println!("{}", row);
        // let row_data: Vec<&str> = row.split(",").collect();
        // let cells: Vec<Cell> = row_data.iter().map(|c| Cell::from(c.to_string())).collect();
        // rows.push(Row::new(cells));
    }
    Table::new(rows)
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

    let course = create_course_from_yaml_file("/home/gerb/uni/courses/SC/SC42150-SSP/");
    get_tasks_for_course(&course);

    Ok(())
}
