use crate::{
   builtins::{cd, echo, exit, pwd, type_cmd},
   commands::command::UserInput,
   utils::{auto_completion::TabCompleter, cmnd_parser, execute_file},
};
use rustyline::{Editor, history::FileHistory};
// use std::cell::Cell;
use std::{cell::RefCell, io::{self, Write}, ops::Deref};
use std::cell::Cell;
use std::rc::Rc;
/*working of this project -Keyboard → Terminal Driver → Rustyline → Your code
model of this code -Keyboard → key events → terminal driver → line editor → program
there is a terminal layer in b/w keyboard and Shell*/

pub fn start() {
   /*taking the user i/p
   -io::stdin() -user i/p pipeline -taking i/p from keyboard
   -read_line -reads an entire line until \n is present -accepts stream of bytes from stdin pipe
   stage27- terminal == canonical mode(line buffered) -> prints immediately the i/p given by the user
   io::stdin().read_line(&mut cmnd.raw).unwrap();-  worked for before stage27
   at stage27 -raw mode is reqd to read the i/p key by key*/
   /*Editor -gives with an editor, replaces stdin.read_line
   2gens- H-TabCompleter(Helper), I-FileHistory(CmndHistory)*/
   let mut rl = Editor::<TabCompleter, FileHistory>::new().unwrap();

   /*registering of autocomplete logic to this Editor*/
   let tab_press = Rc::new(RefCell::new(TabCompleter {
      tab_cnt: Cell::new(0),
   }));
   let tab_completer = Rc::clone(&tab_press);
   let tab_completer = tab_completer.borrow_mut().clone();
   rl.set_helper(Some(tab_completer));

   loop {
      /*line => return val of readline == i/p
      readline- reads key events*/
      // Reset tab count for each new line
      // if let Some(helper) = rl.helper_mut() {
      //    helper.tab_cnt.set(0);
      // }
      tab_press.borrow_mut().tab_cnt.set(0);
      let line = match rl.readline("$ ") {
         Ok(l) => l,
         Err(_) => {
            break;
         }
      };

      /*io::stdout() -pipe to show o/p on terminal, o/p handle of terminal
      -stores in shared buffer(memory) and then pushes to terminal, has a mutex lock for safety
      -unwrap is used to handle exceptional cases
      -this line immediately shows the i/p given on terminal
      -mutex lock- locks the resources/threads until the work is done on and then unlocks it, here the resource is the terminal
      -this line flushes the line print!("$ ") and then accepts the user i/p */
      io::stdout().flush().unwrap();
      let cmnd = UserInput {
         raw: format!("{line}\n"),
      };

      /*stage27-automcomplete -> before parsed args coz the typing of command is still going on -> edits commands and not execute them
      read_line - canonical mode or cooked mode -detects tab as space and not \t
      for detection of \t -raw mode is used*/

      /*stage22 -> parsing logic- separation of concern- parsing done only once*/
      let args = cmnd_parser::handle(&cmnd);
      if args.is_empty() {
         continue;
      }
      /*stage10- run an executable
      stage22 -demands that the execution logic should be kept at last to 1st chk all the builtins and then chk the external commands*/
      if execute_file::handle(&args) {
         continue;
      }
      //parsing the UserInput
      if exit::handle(&args) {
         break;
      }
      if echo::handle(&args) {
         continue;
      }

      if type_cmd::handle(&args) {
         continue;
      }

      //stage11 -pwd
      if pwd::handle(&args) {
         continue;
      }
      //stage 12
      if cd::handle(&args) {
         continue;
      }

      println!("{}: command not found", &args[0]);
      // eprintln!("{:?}", args);
   }
}
