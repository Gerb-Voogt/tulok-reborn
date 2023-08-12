use crate::{*, tasks::get_pending_tasks}; // Probably bad practice but works for now

use std::error::Error;

// Update later such that this is pulled from a config file instead
const COURSES_DIR: &str = "/home/gerb/uni/courses/";

use chrono;
use tui::{
    backend::{CrosstermBackend, Backend},
    widgets::{Widget, Block, Borders, TableState, Table, Cell, Row},
    layout::{Layout, Constraint, Direction, Rect},
    Terminal,
    Frame, text::Span, style::{Style, Color, Modifier},
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{size, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use serde::{Deserialize, Serialize};

pub fn ui<B: Backend>(f: &mut Frame<B>) {
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
        .constraints([Constraint::Percentage(12), Constraint::Percentage(88)].as_ref())
        .split(chunks[1]);

    let chunk_right_top = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(chunk_right[0]);

    // Hard coding this for now but should be parametetric and determined by some
    // application state structure which contains the currently highlighted course
    let course = create_course_from_yaml_file("/home/gerb/uni/courses/SC/SC42150-SSP/");

    draw_course_info_block(&course, f, chunk_left[0]);
    draw_course_operations_block(f, chunk_left[1]);
    draw_current_courses_block(f, chunk_left[2]);
    draw_date_block(f, chunk_right_top[0]);
    draw_task_status_block(f, chunk_right_top[1]);
    draw_tasks_view_block(&course, f, chunk_right[1]);
}


pub fn draw_course_info_block<B>(course: &Course, f: &mut Frame<B>, layout_chunk: Rect)
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


pub fn draw_current_courses_block<B>(f: &mut Frame<B>, layout_chunk: Rect) 
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

pub fn draw_course_operations_block<B>(f: &mut Frame<B>, layout_chunk: Rect)
    where B: Backend {

    let t = Table::new(vec![
        Row::new(vec![Cell::from(format!("Files"))]),
        Row::new(vec![Cell::from(format!("Notes"))]),
        Row::new(vec![Cell::from(format!("Brightspace"))]),
        ])
        .block(Block::default().borders(Borders::ALL).title("Operations"))
        .widths(&[Constraint::Percentage(100)]);

    f.render_widget(t, layout_chunk);
}

pub fn draw_tasks_view_block<B>(course: &Course, f: &mut Frame<B>, layout_chunk: Rect)
    where B: Backend {

    let tasks = get_pending_tasks(course).unwrap();

    let first_row_data: Row = Row::new(vec![
        Cell::from("--"),
        Cell::from("---"),
        Cell::from("-----------"),
        Cell::from("-------"),
    ]);
    let mut rows: Vec<Row> = vec![first_row_data];
    for task in tasks {
        let today = chrono::Utc::now().naive_utc();
        let due_date = task.due();
        let days_until;


        match due_date {
            Some(due_date) => {
                let diff = due_date.signed_duration_since(today);
                days_until = diff.num_days();
            }
            None => { 
                days_until = 0;
            }
        }

        let row_data: Vec<Cell> = vec![
            Cell::from(format!("{}", task.id().unwrap())),
            Cell::from(format!("{}d", days_until)),
            Cell::from(format!("{}", task.description())),
            Cell::from(format!("{}", task.urgency().unwrap())),
        ];
        let row = Row::new(row_data);
        rows.push(row);
    }

    let t = Table::new(rows)
        .block(Block::default().borders(Borders::ALL).title("Up Next"))
        .widths(&[
            Constraint::Percentage(3),
            Constraint::Percentage(7),
            Constraint::Percentage(77),
            Constraint::Percentage(13),
        ])
        .header(Row::new(vec![
            Cell::from("id"),
            Cell::from("due"),
            Cell::from("description"),
            Cell::from("urgency"),
        ]));

    f.render_widget(t, layout_chunk);
}

pub fn draw_date_block<B>(f: &mut Frame<B>, layout_chunk: Rect) 
    where B: Backend {
    let local: chrono::DateTime<chrono::Local> = chrono::Local::now();
    let formatted_date = local.format("%A %d %B %Y %H:%M").to_string();
    let t = Table::new(vec![
        Row::new(vec![Cell::from(format!("{}", formatted_date))]),
        ])
        .block(Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD))
            .title("Today is"))
        .widths(&[Constraint::Percentage(100)]);
    f.render_widget(t, layout_chunk);
}

pub fn draw_task_status_block<B>(f: &mut Frame<B>, layout_chunk: Rect) 
    where B: Backend {

    let task_total_count = String::from_utf8(
        process::Command::new("sh")
            .arg("-c")
            .arg("task rc.verbose=none project:uni +PENDING count | tr -s ' ' | tr -d '\n'")
            .output()
            .expect("Failed to run command")
        .stdout)
        .unwrap();
    let task_ready_count = String::from_utf8(
        process::Command::new("sh")
            .arg("-c")
            .arg("task rc.verbose=none project:uni +READY count | tr -s ' ' | tr -d '\n'")
            .output()
            .expect("Failed to run command")
        .stdout)
        .unwrap();
    let task_scheduled_count = String::from_utf8(
        process::Command::new("sh")
            .arg("-c")
            .arg("task rc.verbose=none project:uni +SCHEDULED count | tr -s ' ' | tr -d '\n'")
            .output()
            .expect("Failed to run command")
        .stdout)
        .unwrap();
    let task_blocked_count = String::from_utf8(
        process::Command::new("sh")
            .arg("-c")
            .arg("task rc.verbose=none project:uni +BLOCKED count | tr -s ' ' | tr -d '\n'")
            .output()
            .expect("Failed to run command")
        .stdout)
        .unwrap();

    let task_status_string = format!("Total: {} / Ready: {} / Scheduled: {} / Blocked: {}",
            task_total_count,
            task_ready_count,
            task_scheduled_count,
            task_blocked_count);

    let t = Table::new(vec![
        Row::new(vec![Cell::from(task_status_string)]),
    ])
        .block(Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD))
            .title("Uni Tasks State"))
        .widths(&[Constraint::Percentage(100)]);
    f.render_widget(t, layout_chunk);
}
