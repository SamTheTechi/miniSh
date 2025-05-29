use mini_sh::builtin::*;
use std::collections::HashMap;
use std::io::{Write, stdin, stdout};

fn main() {
    let mut hashmap: HashMap<String, String> = HashMap::new();
    let _ = mini_sh::process_minishrc(&mut hashmap);

    println!("Welcome to mini_sh!!");
    loop {
        print!("-→ ");
        stdout()
            .flush()
            .unwrap_or_else(|e| eprintln!("Error flushing stdout: {}", e));

        let mut input: String = String::new();
        let _ = stdin().read_line(&mut input).unwrap();
        let mut commands = input.trim().split(";").peekable();

        while let Some(command) = commands.next() {
            let command = command.trim();
            if command.is_empty() {
                continue;
            }

            let mut parts = command.splitn(2, ' ');
            let cmd_word = parts.next().unwrap();
            let cmd_args = parts.next().unwrap_or("").trim();
            let expanded_command = hashmap
                .get(cmd_word)
                .map(|s| s.as_str())
                .unwrap_or(cmd_word);

            match expanded_command {
                "cd" => {
                    change_directory::run(&cmd_args);
                }
                "exports" => {
                    export_path::run(&command);
                }
                "alias" => {
                    aliase::run(&command, &mut hashmap);
                }
                "exit" => unsafe {
                    libc::exit(0);
                },
                _ => {
                    execute_command::run(&expanded_command);
                }
            }
        }
    }
}
