use std::io::{self, pipe};
use std::process::{Command, Stdio};
pub fn handle(user_input: &[String]) {
   let pipe_symbol = "|".to_string();

   if user_input.contains(&pipe_symbol) {
      if let Some(pipe_idx) = user_input.iter().position(|p| p == &pipe_symbol) {
         let elems_before_pipe = &user_input[0..pipe_idx];
         let elems_after_pipe = &user_input[pipe_idx + 1..];

         spawn_processes(elems_before_pipe, elems_after_pipe);
      }
   }
}
fn spawn_processes(cmd1: &[String], cmd2: &[String]) {
   if let Ok(ends) = pipe() {
      let (reader, writer) = ends;
      if cmd1.is_empty() || cmd2.is_empty() {
         return;
      }
      let mut child1 = Command::new(&cmd1[0]);
      if cmd1.len() >= 2 {
         child1.args(&cmd1[1..]);
      }
      child1.stdout(Stdio::from(writer));

      let mut child2 = Command::new(&cmd2[0]);
      if cmd2.len() >= 2 {
         child2.args(&cmd2[1..]);
      }
      child2.stdin(Stdio::from(reader));
      // println!("cmd1: {:?}", cmd1);
      // println!("cmd2: {:?}", cmd2);
      let c1 = child1.spawn();
      let c2 = child2.spawn();
      
      match c1 {
         Ok(mut c) => {
            let _ = c.wait();
         }
         _ => {
            match c2 {
               Ok(mut c) => {
                  let _ = c.wait();
               }
               _=>{}
            }
         }
      }
   }
}
