use crate::commands::command::UserInput;
use crate::utils::path::find_executable;
pub fn handle(cmd: &UserInput) -> bool {
    let cmd = cmd.raw.trim();
    let command_to_be_printed = match cmd.trim().strip_prefix("type ") {
        Some(c) => c.trim(),
        None => return false,
    };

    let builtins = ["echo", "exit", "type"];

    // Builtin check
    if builtins.contains(&command_to_be_printed) {
        println!("{command_to_be_printed} is a shell builtin");
        return true;
    }

    if let Some(file) = find_executable(command_to_be_printed) {
        println!("{command_to_be_printed} is {}", file.display());
    } else {
        println!("{command_to_be_printed}: not found");
    }
    true
}
