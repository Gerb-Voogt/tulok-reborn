use crate::course::Course;

use std::error::Error;
use std::io::{stdout, stdin, BufRead, BufReader, Write};
use std::process::Stdio;
use task_hookrs::status::TaskStatus;
use task_hookrs::task::{Task, TW26};
use task_hookrs::import::import;
use tui::style::{Style, Color};

pub fn get_pending_tasks(course: &Course) -> std::io::Result<Vec<Task>> {
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

pub fn get_all_pending_tasks() -> std::io::Result<Vec<Task>> {
    let command = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!("task +PENDING project:uni or project:work export"))
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


fn has_pending_dependency(task: Task) -> bool {
    // Thanks ChatGPT!
    let tasks_list = get_all_pending_tasks().unwrap();
    let pending_task_uuids: Vec<String> = tasks_list.iter()
        .map(|t| t.uuid().to_string())
        .collect();

    if let Some(dependent_uuids) = task.depends() {
        let dependent_uuids_as_string: Vec<String> = dependent_uuids.iter()
            .map(|t| t.to_string())
            .collect();
        
        for uuid in dependent_uuids_as_string {
            if pending_task_uuids.contains(&uuid) {
                return true;
            }
        }
    }

    false
}

pub fn get_task_styling(task: Task) -> Style {
    if has_pending_dependency(task) == true {
        Style::default().fg(Color::Reset).bg(Color::DarkGray)
    } else {
        Style::default().fg(Color::Black).bg(Color::Gray)
    }
}

fn log(msg: &str) -> std::io::Result<()> {
    let log_file_path = "/home/gerb/uni/dev/tulok-reborn/log_file.log";
    let mut log_file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(log_file_path)
        .unwrap();
    log_file.write_all(b"\n")?;
    log_file.write_all(msg.as_bytes())?;

    Ok(())
}
