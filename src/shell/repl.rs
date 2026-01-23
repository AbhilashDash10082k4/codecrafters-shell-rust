use crate::builtins::{cd, echo, exit, pwd, type_cmd};
use crate::commands::command::UserInput;
use crate::utils::{cmnd_parser, execute_file};
use std::io::{self, Write};
pub fn start() {
    loop {
        //no \n so no automatic flushing , requires manual flushing
        print!("$ ");

        /*io::stdout() -pipe to show o/p on terminal, o/p handle of terminal
        -stores in shared buffer(memory) and then pushes to terminal, has a mutex lock for safety
        -unwrap is used to handle exceptional cases
        -this line immediately shows the i/p given on terminal
        -mutex lock- locks the resources/threads until the work is done on and then unlocks it, here the resource is the terminal
        -this line flushes the line print!("$ ") and then accepts the user i/p */
        io::stdout().flush().unwrap();
        let mut cmnd = UserInput::new();

        /*taking the user i/p
        -io::stdin() -user i/p pipeline -taking i/p from keyboard
        -read_line -reads an entire line until \n is present
        */
        io::stdin().read_line(&mut cmnd.raw).unwrap();

        //parsing the UserInput
        if exit::handle(&cmnd) {
            break;
        }

        if echo::handle(&cmnd) {
            continue;
        }

        if type_cmd::handle(&cmnd) {
            continue;
        }
        //stage10- run an executable
        if execute_file::handle(&cmnd) {
            continue;
        }
        //stage11 -pwd
        if pwd::handle(&cmnd) {
            continue;
        }
        //stage 12
        if cd::handle(&cmnd) {
            continue;
        }
        /*stage22 anything except builtin -execute it*/
        execute_file::handle(&cmnd);
        // println!("{}: command not found", cmnd.raw.trim());
    }
}
