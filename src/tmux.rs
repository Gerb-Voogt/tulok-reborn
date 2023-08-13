use std::{process, fs};

pub fn open_tmux_window() {
    unimplemented!()
}

pub fn open_tmux_session(session_name: &str, path: fs::DirEntry) {
    let path_string = path.path().display().to_string();
    let tmux_command = format!("tmux new-session -ds {} -c {}", 
        session_name,
        path_string);

    process::Command::new("sh")
        .arg("-c")
        .arg(tmux_command)
        .output()
        .expect("Failed to run command");
}

pub fn switch_to_tmux_session(session_name: &str) {
    let switch_session_command = format!("tmux switch-client -t {}", 
        session_name);
    process::Command::new("sh")
        .arg("-c")
        .arg(switch_session_command)
        .output()
        .expect("Failed to run command");
}
