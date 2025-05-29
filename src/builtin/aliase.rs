use crate::strip_quotes;
use std::collections::HashMap;

pub fn run(command: &str, hashmap: &mut HashMap<String, String>) {
    let command = command.trim();
    if let Some((c_command, args)) = command.split_once(" ") {
        if c_command == "alias" {
            let c_args: Vec<String> = args
                .splitn(2, "=")
                .map(|s| strip_quotes(s).trim().to_string())
                .collect();

            if c_args.len() != 2 {
                eprintln!("Invalid alias syntax. Expected: alias VAR=VALUE");
                return;
            }

            let key = c_args[0].clone();
            let value = c_args[1].clone();

            hashmap.insert(key, value);
        } else {
            eprintln!("Invalid aliase syntax!: {}", command);
        }
    } else {
        eprintln!("No arguments provided");
    }
}
