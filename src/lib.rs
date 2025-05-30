pub mod builtin;
use crate::builtin::*;
use std::{collections::HashMap, env, fs};

pub fn strip_quotes(s: &str) -> &str {
    let s = s.trim();
    if ((s.starts_with('"') && s.ends_with('"')) || (s.starts_with("'") && s.ends_with("'")))
        && s.len() >= 2
    {
        &s[1..s.len() - 1]
    } else {
        s
    }
}

pub fn process_minishrc(hashmap: &mut HashMap<String, String>) {
    let mut path = match env::home_dir() {
        Some(path) => path,
        None => {
            eprintln!("Error Could not resolve use's home directory. Cannot load minish.conf");
            return;
        }
    };

    path.push("minish.conf");

    let content_str = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Could not read file at {:?}", path);
            return;
        }
    };

    let unrefine_commands = content_str
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty() && !s.starts_with('#'));

    for cmd in unrefine_commands {
        if cmd.starts_with("alias ") {
            aliase::run(&cmd, hashmap);
        } else if cmd.starts_with("exports ") {
            export_path::run(&cmd);
        } else {
            eprintln!("Unknown command: {}", cmd);
        }
    }
}
