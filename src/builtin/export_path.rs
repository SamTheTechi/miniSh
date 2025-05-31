use std::env;

pub fn run(args: &str) {
    let args = args.trim();
    if !args.is_empty() {
        let c_args: Vec<&str> = args.splitn(2, "=").collect();

        if c_args.len() != 2 {
            eprintln!("exports: Invalid export syntax. Expected: exports VAR=VALUE");
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
                eprintln!("exports: Environment variable '{}' not found", key);
                unsafe {
                    env::set_var(key, value);
                }
            }
        }
    } else {
        eprintln!("exports: No arguments provided");
    }
}
