use crate::ConfigCommand;
use std::{env, fs, path::PathBuf};

pub fn process_minishrc() {
    // getting users's home directory if not exists this take root
    let mut path = env::home_dir().unwrap_or(PathBuf::from("/"));

    // concatincation of paths
    let config_file = "minish.conf";
    path.push(config_file);

    let content_str = fs::read_to_string(&path).unwrap();

    let str = commands_parser(content_str);
}

fn commands_parser(str: String) -> Vec<ConfigCommand> {
    let mut vec: Vec<ConfigCommand> = Vec::new();
    let unrefine_commands: Vec<String> = str
        .split('\n')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty() && !s.starts_with('#'))
        .collect();

    for commands in unrefine_commands {
        let cmd = commands.trim();

        if cmd.starts_with("alias ") {
            let sections: Vec<&str> = cmd["alias ".len()..].splitn(2, '=').collect();
            let new_config_command = ConfigCommand{
                Alias(sections[0], sections[1])
            }
            vec.push(new_config_command);
        } else if cmd.starts_with("exports ") {
            let sections: Vec<&str> = cmd["exports ".len()..].splitn(2, '=').collect();
            let new_config_command = ConfigCommand{
                Alias(sections[0], sections[1])
            }
            vec.push(new_config_command);
        } else {
            eprint!("error ");
        }
    }

    vec
}
