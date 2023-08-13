use std::process;

use crate::course::{
    self,
    Course,
    retrieve_courses_active,
    COURSES_DIR
};
use crate::tmux;

use tui::widgets::TableState;

#[derive(Debug)]
pub enum RouteId {
}

#[derive(Debug)]
pub enum ActiveBlock {
}

#[derive(Debug)]
pub struct Route {
    pub id: RouteId,
    pub active_block: ActiveBlock,
    pub hovered_block: ActiveBlock,
}

pub struct App {
    pub highlighted_course: Course,
    pub active_courses: Vec<Course>,
    pub state: TableState,
}

impl App {
    pub fn new() -> App {
        let active_courses_list = retrieve_courses_active(COURSES_DIR);
        let first_course = &active_courses_list[0];
        App {
            highlighted_course: first_course.clone(),
            active_courses: active_courses_list,
            state: TableState::default(),
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.active_courses.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.update_highlighted_course();
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.active_courses.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.update_highlighted_course();
    }

    pub fn update_highlighted_course(&mut self) {
        let i = self.state.selected().unwrap_or_default();
        let course_list = self.active_courses.clone();
        self.highlighted_course = course_list[i].clone();
    }

    pub fn open_notes(&self) {
        let notes_dir = self.highlighted_course
            .find_notes_dir()
            .unwrap();
        let tmux_session_name = format!("{}-Notes", self.highlighted_course.code);
        tmux::open_tmux_session(&tmux_session_name, notes_dir);
        tmux::switch_to_tmux_session(&tmux_session_name)
    }

    pub fn open_files(&self) {
        let files_dir = self.highlighted_course
            .find_files_dir()
            .unwrap();
        let tmux_session_name = format!("{}-Files", self.highlighted_course.code);
        tmux::open_tmux_session(&tmux_session_name, files_dir);
        tmux::switch_to_tmux_session(&tmux_session_name)
    }

    pub fn open_brightspace(&self) {
        process::Command::new("sh")
            .arg("-c")
            .arg(format!("firefox {}", self.highlighted_course.url))
            .output()
            .expect("Failed to run command!");
    }
}
