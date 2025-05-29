use std::collections::HashMap;
use std::io::{Write, stdin, stdout};

fn main() {
    println!("Welcome to mini_sh!!");
    crate::process_minishrc();

    loop {
        print!("$ ");
        stdout().flush().unwrap();

        let mut input: String = String::new();
        let _ = stdin().read_line(&mut input).unwrap();
        let mut commands = input.trim().split(";").peekable();

        while let Some(command) = commands.next() {
            let command = command.trim();

            match command {
                "cd" => {
                    crate::change_directory(&command);
                }
                "exit" => unsafe {
                    libc::exit(1);
                },
                _ => {
                    crate::execute_command(&command);
                }
            }
        }
    }
}
