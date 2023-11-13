use crate::{
    tasks::{get_pending_tasks, get_task_styling},
    course::*,
    create_course_from_yaml_file,
    app::App,
}; // Probably bad practice but works for now

use std::error::Error;
use std::process;

// Update later such that this is pulled from a config file instead

use chrono;
use tui::{
    backend::{CrosstermBackend, Backend},
    widgets::{Widget, Block, Borders, TableState, Table, Cell, Row},
    layout::{Layout, Constraint, Direction, Rect},
    Terminal,
    Frame, text::{Span, Spans}, style::{Style, Color, Modifier},
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{size, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use serde::{Deserialize, Serialize};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(f.size());


    let chunk_left = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(45), Constraint::Percentage(25)].as_ref())
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
    // let course = create_course_from_yaml_file("/home/gerb/uni/courses/SC/SC42150-SSP/");
    let course = &app.highlighted_course.clone();

    draw_course_info_block(&course, f, chunk_left[0]);
    draw_current_courses_block(f, chunk_left[1], app);
    draw_course_operations_block(f, chunk_left[2]);
    draw_date_block(f, chunk_right_top[0]);
    draw_task_status_block(f, chunk_right_top[1]);
    draw_tasks_view_block(&course, f, chunk_right[1]);
}


pub fn draw_course_info_block<B>(course: &Course, f: &mut Frame<B>, layout_chunk: Rect)
    where B: Backend {

    let course_code_styled = Spans::from(vec![
        Span::raw("Code: "),
        Span::styled(course.code.to_string(), Style::default().add_modifier(Modifier::BOLD))
    ]);
    let course_title_styled = Spans::from(vec![
        Span::raw("Title: "),
        Span::styled(course.title.to_string(), Style::default().add_modifier(Modifier::BOLD))
    ]);
    let course_year_styled = Spans::from(vec![
        Span::raw("Year: "),
        Span::styled(course.year.to_string(), Style::default().add_modifier(Modifier::BOLD))
    ]);
    let course_quarter_styled = Spans::from(vec![
        Span::raw("Quarter: "),
        Span::styled(course.quarter.to_string(), Style::default().add_modifier(Modifier::BOLD))
    ]);
    let course_active_styled = Spans::from(vec![
        Span::raw("Active: "),
        Span::styled(course.active.to_string(), Style::default().add_modifier(Modifier::BOLD))
    ]);
    let course_credits_styled = Spans::from(vec![
        Span::raw("ECTS: "),
        Span::styled(course.credits.to_string(), Style::default().add_modifier(Modifier::BOLD))
    ]);

    let t = Table::new(vec![
        Row::new(vec![Cell::from(course_code_styled)]),
        Row::new(vec![Cell::from(course_title_styled)]),
        Row::new(vec![Cell::from(course_year_styled)]),
        Row::new(vec![Cell::from(course_quarter_styled)]),
        Row::new(vec![Cell::from(course_active_styled)]),
        Row::new(vec![Cell::from(course_credits_styled)]),
        ])
        .block(Block::default().borders(Borders::ALL).title("Course Info"))
        .widths(&[Constraint::Percentage(100)]);

    f.render_widget(t, layout_chunk);
}


pub fn draw_current_courses_block<B>(f: &mut Frame<B>, layout_chunk: Rect, app: &mut App) 
    where B: Backend {

    let mut rows: Vec<Row> = Vec::new();
    let active_courses = retrieve_courses_active(COURSES_DIR);
    let active_courses = Course::sort_by_quarter(active_courses);

    let selected_style = Style::default().add_modifier(Modifier::BOLD);

    for course in active_courses {
        rows.push(
            Row::new(
                vec![
                    Cell::from(format!("{}-{}", course.code, course.short))
                        .style(Style::default().fg(get_color_for_course_code(course)))
                ])
        );
    }
    let t = Table::new(rows)
        .block(Block::default().borders(Borders::ALL).title("Courses"))
        .highlight_style(selected_style)
        .widths(&[Constraint::Percentage(100)]);
    f.render_stateful_widget(t, layout_chunk, &mut app.state);
}

pub fn draw_course_operations_block<B>(f: &mut Frame<B>, layout_chunk: Rect)
    where B: Backend {

    let t = Table::new(vec![
        Row::new(vec![Cell::from(format!("[F]iles"))]),
        Row::new(vec![Cell::from(format!("[N]otes"))]),
        Row::new(vec![Cell::from(format!("[B]rightspace"))]),
        Row::new(vec![Cell::from(format!("[q]uit"))]),
        ])
        .block(Block::default().borders(Borders::ALL).title("Operations"))
        .widths(&[Constraint::Percentage(100)]);

    f.render_widget(t, layout_chunk);
}

// This function does too much. Refactor such that the calculation of the days until due date
// happens in a seperate function.
pub fn draw_tasks_view_block<B>(course: &Course, f: &mut Frame<B>, layout_chunk: Rect)
    where B: Backend {

    let tasks = get_pending_tasks(course).unwrap();

    let first_row_data: Row = Row::new(vec![
        Cell::from("--"),
        Cell::from("---"),
        Cell::from("-----------"),
        Cell::from("-------"),
    ]).style(Style::default().fg(Color::Reset));
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
        let row = Row::new(row_data).style(get_task_styling(task));
        rows.push(row);
    }

    let t = Table::new(rows)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Up Next")
            .style(Style::default().fg(Color::Reset)))
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
        ]).style(Style::default().fg(Color::Reset)));

    f.render_widget(t, layout_chunk);
}

pub fn draw_date_block<B>(f: &mut Frame<B>, layout_chunk: Rect) 
    where B: Backend {
    let local: chrono::DateTime<chrono::Local> = chrono::Local::now();
    let formatted_date = local.format("%A %d %B %Y %H:%M").to_string();
    let t = Table::new(vec![
        Row::new(vec![
            Cell::from(format!("{}", formatted_date))
                .style(Style::default().fg(Color::Reset))
        ]),
        ])
        .block(Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Reset))
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
        Row::new(vec![
            Cell::from(task_status_string)
                .style(Style::default().fg(Color::Reset))
        ]),
    ])
        .block(Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Reset))
            .title("Uni Tasks State"))
        .widths(&[Constraint::Percentage(100)]);
    f.render_widget(t, layout_chunk);
}
