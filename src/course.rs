use std::{
    fs,
    fs::DirEntry,
};
use serde::{Deserialize, Serialize};
use tui::style::Color;
use regex::Regex;

pub const COURSES_DIR: &str = "/home/gerb/uni/courses/";
pub const NOTES_DIR: &str = "/home/gerb/uni/Vault-MSc/";

// Deriving these allows for automagically importing the yaml files
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)] 
pub struct Course {
    pub title: String,
    pub short: String,
    pub code: String,
    pub year: String,
    pub quarter: String,
    pub url: String,
    pub active: bool,
}

impl Course {
    pub fn find_notes_dir(&self) -> Option<DirEntry> {
        self.find_matching_dirs_recursive(NOTES_DIR)
    }

    pub fn find_files_dir(&self) -> Option<DirEntry> {
        self.find_matching_dirs_recursive(COURSES_DIR)
    }

    fn find_matching_dirs_recursive(&self, search_path: &str) -> Option<DirEntry> {
        let base_dir = std::path::Path::new(search_path);
        let top_level_dirs = fs::read_dir(base_dir).unwrap();
        
        for entry in top_level_dirs {
            let path = entry.unwrap().path();
            let metadata = fs::metadata(&path).unwrap();

            if metadata.is_file() {
                continue;
            } else {
                let sub_dirs = fs::read_dir(path).unwrap();
                for dir in sub_dirs {
                    let search_string = format!("{}-{}", &self.code, &self.short);
                    let dir = dir.unwrap();
                    if dir.path().display().to_string().contains(&search_string) {
                        return Some(dir);
                    }
                }
            }
        }
        None
    }
}

/// Note that in the current state this function may panic
/// it would be better to rewrite this such that the error is handled explicitly
/// rather then the program crashing outright
/// 
/// It might also be better to make it such that this function
/// accepts an actual path object rather then a string.
pub fn create_course_from_yaml_file(course_path: &str) -> Course {
    // Read the file
    let f = std::fs::File::open(course_path.to_owned() + "/info.yaml").expect("Could not read file!");
    serde_yaml::from_reader(f).expect("Could not read file!")
}

/// This function may panic. Should make it so explictly return a Result type
/// This function is also kinda gross. Need to figure out whether there is a better way to do this
/// because it has unwraps all over the place.
pub fn retrieve_courses_active(courses_dir: &str) -> Vec<Course> {
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


pub fn get_color_for_course_code(course: Course) -> Color {
    let re = Regex::new(r"([A-Z]+)").unwrap();
    let course_code_string = &course.code;
    let caps = re.captures(course_code_string).unwrap();

    let prefix = caps.get(0).unwrap().as_str(); // Unwrap here because formatting should be consistent
    match prefix {
        "AM" | "TW" | "WI" => Color::Magenta,
        "CESE" | "EE" | "ET" => Color::Red,
        "CS" | "IN" | "CSE" => Color::Cyan,
        "WB" | "ME" => Color::Yellow,
        "SC" => Color::Blue,
        "RO" => Color::Green,
        _ => Color::Gray,
    }
}
