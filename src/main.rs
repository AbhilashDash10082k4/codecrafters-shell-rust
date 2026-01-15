use std::io::{self, Write};
mod user_command;
use crate::user_command::command::Command;
fn main() {
    loop {
        //no \n so no automatic flushing , requires manual flushing
        print!("$ ");

        /*io::stdout() -pipe to show o/p on terminal, o/p handle of terminal
        -stores in shared buffer(memory) and then pushes to terminal, has a mutex lock for safety
        -unwrap is used to handle exceptional cases
        -this line immediately shows the i/p given on terminal
        -mutex lock- locks the resources/threads until the work is done on and then unlocks it, here the resource is the terminal
        */
        io::stdout().flush().unwrap();

        let mut cmnd = Command {
            command: String::new(),
        };

        /*taking the user i/p
        -io::stdin() -user i/p pipeline -taking i/p from keyboard
        -read_line -reads an entire line until \n is present*/
        io::stdin().read_line(&mut cmnd.command).unwrap();

        //parsing the command
        if cmnd.exit() {
            break;
        }
        let (echo, args) = cmnd.echo();
        if echo == "echo" {
            println!("{}", args);
            continue;
        }

        if cmnd.command.trim().contains("type ") {
            cmnd.type_command();
            continue;
        }
        // search_file_in_path();
        println!("{}: command not found", cmnd.command.trim());
    }
}
