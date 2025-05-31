use crate::strip_quotes;
use std::collections::HashMap;

pub fn run(args: &str, hashmap: &mut HashMap<String, String>) {
    let args = args.trim();
    if !args.is_empty() {
        let c_args: Vec<String> = args
            .splitn(2, "=")
            .map(|s| strip_quotes(s).trim().to_string())
            .collect();

        if c_args.len() != 2 {
            eprintln!("alias : Invalid alias syntax. Expected: alias VAR=VALUE");
            return;
        }

        let key = c_args[0].clone();
        let value = c_args[1].clone();

        hashmap.insert(key, value);
    } else {
        eprintln!("alias: No arguments provided");
    }
}
