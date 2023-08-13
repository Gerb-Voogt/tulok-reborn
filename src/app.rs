use crate::course::{self, Course, retrieve_courses_active, COURSES_DIR};
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
        self.state.select(Some(i))
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
        self.state.select(Some(i))
    }

    pub fn update_highlighted_course(&mut self) {
        let i = self.state.selected().unwrap_or_default();
        let course_list = self.active_courses.clone();
        self.highlighted_course = course_list[i].clone();
    }
}
