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

            let original_cmd = parts.next().unwrap();
            let original_args = parts.next().unwrap_or("").trim();

            let expanded = hashmap
                .get(original_cmd)
                .map(|s| s.as_str())
                .unwrap_or(original_cmd);

            let mut new_parts = expanded.splitn(2, ' ');

            let real_cmd = new_parts.next().unwrap();
            let alias_args = new_parts.next().unwrap_or("").trim();

            let combined_args = if alias_args.is_empty() {
                original_args.to_string()
            } else if original_args.is_empty() {
                alias_args.to_string()
            } else {
                format!("{} {}", alias_args, original_args)
            };

            match real_cmd {
                "cd" => {
                    change_directory::run(&combined_args);
                }
                "exports" => {
                    export_path::run(&format!("{} {}", real_cmd, combined_args));
                }
                "alias" => {
                    aliase::run(&format!("{} {}", real_cmd, combined_args), &mut hashmap);
                }
                "exit" => unsafe {
                    libc::exit(0);
                },
                _ => {
                    execute_command::run(&format!("{} {}", real_cmd, combined_args));
                }
            }
        }
    }
}
