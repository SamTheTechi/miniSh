use std::{env, path::Path};

pub fn run(command: &str) {
    let command: Vec<&str> = command.trim().split_whitespace().collect();
    match command.len() {
        0 => {
            let new_home_path = env::var("HOME").unwrap_or_else(|_| "/".to_string());
            if let Err(e) = env::set_current_dir(new_home_path) {
                eprintln!("Error changing directory: {}", e);
            }
        }
        1 => {
            let new_path = command[0].trim();
            let root = Path::new(new_path);
            if let Err(e) = env::set_current_dir(root) {
                eprintln!("Error changing directory: {}", e);
            }
        }
        _ => {
            eprintln!("cd: string not in pwd: {}", command[0]);
            return;
        }
    }
}
