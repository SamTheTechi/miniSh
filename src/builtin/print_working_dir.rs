use crate::strip_quotes;
use std::env;

pub fn run(args: &str) {
    if args.is_empty() {
        match env::current_dir() {
            Ok(unrefined_path) => {
                let path = strip_quotes(unrefined_path.to_str().unwrap());
                println!("{:?}", path);
            }
            Err(e) => {
                eprintln!("pwd: error fetching path: {}", e);
            }
        }
    } else {
        eprintln!("pwd: too many arguments");
    }
}
