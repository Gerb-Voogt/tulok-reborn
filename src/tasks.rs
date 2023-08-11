use crate::course::Course;

use std::error::Error;
use std::io::{stdout, stdin, BufRead, BufReader};
use std::process::Stdio;
use task_hookrs::status::TaskStatus;
use task_hookrs::task::{Task, TW26};
use task_hookrs::import::import;

pub fn get_pending_tasks(course: &Course) -> Result<Vec<Task>, Box<dyn Error>> {
    let command = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!("task +{} export", course.code))
        .output()
        .expect("Something went wrong");
    let stdout_reader = BufReader::new(&command.stdout[..]);
    let tasks = import::<TW26, _>(stdout_reader);

    let mut tasks_list: Vec<Task> = Vec::new();

    match tasks {
        Ok(tasks) => {
            for task in tasks {
                if *task.status() == TaskStatus::Pending {
                    tasks_list.push(task);
                }
            }
        }
        Err(err) => println!("Error Importing tasks: {}", err)
    }
    Ok(tasks_list)
}
