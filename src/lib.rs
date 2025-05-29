mod change_directory;
mod execute_command;
mod parse;

pub enum ConfigCommand {
    Alias(String, String),
    Export(String, String),
}
