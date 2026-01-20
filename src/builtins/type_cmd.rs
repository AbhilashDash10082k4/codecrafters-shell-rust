use crate::commands::command::UserInput;
use crate::utils::path::find_executable;
pub fn handle(cmd: &UserInput) -> bool {
    let cmd = cmd.raw.trim();
    let command_to_be_printed = match cmd.trim().strip_prefix("type ") {
        Some(c) => c.trim(),
        None => return false,
    };

    let builtins = ["echo", "exit", "type", "pwd", "cd"];

    // Builtin check
    if builtins.contains(&command_to_be_printed) {
        println!("{command_to_be_printed} is a shell builtin");
        return true;
    }

    /* if let Some syntax -> syntactic sugar over match expression -no need to handle None cases independently.The else part = None/Err arm. Used for complex control flows and prevents immediate return
    
    let else - only handles the else part. If the let condition becomes true, it assigns val to the var defined in the let arm and then other code in the fn are executed. Immediately returns from the fn after failure
    */
    if let Some(file) = find_executable(&command_to_be_printed.to_string()) {
        println!("{command_to_be_printed} is {}", file.display());
    } else {
        println!("{command_to_be_printed}: not found");
    }
    true
}
