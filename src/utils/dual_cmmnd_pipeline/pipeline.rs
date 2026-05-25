use std::io::{Write, pipe, stdout};
use std::process::{Command, Stdio};

use crate::builtins::builtins::{BUILTINS, is_builtin};
use crate::builtins::{echo, pwd, type_cmd};

pub fn handle(user_input: &[String]) {
   let pipe_symbol = "|".to_string();

   if user_input.contains(&pipe_symbol) {
      if let Some(pipe_idx) = user_input.iter().position(|p| p == &pipe_symbol) {
         let elems_before_pipe = &user_input[0..pipe_idx];
         let elems_after_pipe = &user_input[pipe_idx + 1..];

         execute_cmnd(elems_before_pipe, elems_after_pipe);
      }
   }
}

fn child_process_creation(cmd: &[String]) -> Command {
   let mut child = Command::new(&cmd[0]);
   if cmd.len() >= 2 {
      child.args(&cmd[1..]);
   }
   child
}
fn builtin_executor(cmd: &[String], out: &mut impl Write) -> bool {
   let builtins = BUILTINS;
   for builtin in builtins {
      /*builtins should directly execute and not spawn any external processes
      external command spawn new processes
      but both should still support the stdin/stdout redirection*/
      //left side execution
      if cmd.contains(&builtin.to_string()) {
         match builtin {
            "echo" => {
               return echo::handle(cmd, out);
            }
            "pwd" => {
               return pwd::handle(cmd, out);
            }
            "type" => {
               return type_cmd::handle(cmd, out);
            }
            _ => {}
         }
      }
   }
   false
}

fn execute_cmnd(cmd1: &[String], cmd2: &[String]) {
   if let Ok(ends) = pipe() {
      let (reader, writer) = ends;
      if cmd1.is_empty() || cmd2.is_empty() {
         return;
      }

      if is_builtin(cmd1) || is_builtin(cmd2) {
         //for left of pipe
         let mut writer = &writer;
         builtin_executor(cmd1, &mut writer);

         //for right of pipe

         let mut out = stdout();
         builtin_executor(cmd2, &mut out);
      } else {
         let mut child1 = child_process_creation(cmd1);
         child1.stdout(Stdio::from(writer));
         let Ok(mut c1) = child1.spawn() else {
            return;
         };

         let mut child2 = child_process_creation(cmd2);
         child2.stdin(Stdio::from(reader));

         let Ok(mut c2) = child2.spawn() else {
            return;
         };

         let _ = c1.wait();
         let _ = c2.wait();
      }
   }
}
