//outside a module const is used and not let
pub const BUILTINS: [&str; 5] = ["echo", "exit", "type", "cd", "pwd"];
pub fn is_builtin(cmd: &[String]) -> bool {
   if cmd.is_empty() {
      return false;
   }
   if BUILTINS.contains(&cmd[0].as_str()) {
      return true;
   } else {
      return false;
   }
}
