use crate::commands::command::UserInput;
pub fn handle(cmd: &UserInput) -> bool {
    if cmd.raw.trim() == "exit" {
        return true;
    } else {
        return false;
    }
}
