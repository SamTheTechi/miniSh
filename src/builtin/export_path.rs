use std::env;

pub fn run(command: &str) {
    let command = command.trim();
    if let Some((c_command, args)) = command.split_once(" ") {
        if c_command == "exports" {
            let c_args: Vec<&str> = args.splitn(2, "=").collect();

            if c_args.len() != 2 {
                eprintln!("Invalid export syntax. Expected: exports VAR=VALUE");
                return;
            }

            let key = c_args[0].trim();
            let value = c_args[1].trim();

            match env::var(key) {
                Ok(current_path_env) => {
                    let new_path_env = format!("{}:{}", current_path_env, value);
                    unsafe {
                        env::set_var(key, new_path_env);
                    }
                }
                Err(_) => {
                    eprintln!("Warning: Environment variable '{}' not found", key);
                    unsafe {
                        env::set_var(key, value);
                    }
                }
            }
        } else {
            eprintln!("Invalid  export syntax!: {}", command);
        }
    } else {
        eprintln!("No arguments provided");
    }
}
