use std::fs;
use serde::{Deserialize, Serialize};

// Deriving these allows for automagically importing the yaml files
#[derive(Serialize, Deserialize)] 
pub struct Course {
    pub title: String,
    pub short: String,
    pub code: String,
    pub year: String,
    pub quarter: String,
    pub url: String,
    pub active: bool,
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

