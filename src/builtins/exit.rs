use crate::commands::command::Command;
pub fn handle(cmd: &Command) -> bool {
    if cmd.raw.trim() == "exit" {
        return true;
    } else {
        return false;
    }
}
