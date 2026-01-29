use crate::builtins::{cd, echo, exit, pwd, type_cmd};
use crate::commands::command::UserInput;
use crate::utils::{auto_completion, cmnd_parser, execute_file};
use rustyline::DefaultEditor;
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
        let cmnd = UserInput::new();

        /*taking the user i/p
        -io::stdin() -user i/p pipeline -taking i/p from keyboard
        -read_line -reads an entire line until \n is present -accepts stream of bytes from stdin pipe
        stage27- terminal == canonical mode(line buffered) -> prints immediately the i/p given by the user
        io::stdin().read_line(&mut cmnd.raw).unwrap();-  worked for before stage27
        at stage27 -raw mode is reqd to read teh i/p key by key*/
        let rl = DefaultEditor::new();
        match rl {
            Ok(mut s) => {
                let _= s.readline(&cmnd.raw);
            }
            _ => {
                return;
            }
        }
        /*stage27-automcomplete -> before parsed args coz the typing of command is still going on -> edits commands and not execute them
        read_line - canonical mode or cooked mode -detects tab as space and not \t
        for detection of \t -raw mode is used*/
        if auto_completion::handle(&cmnd.raw) {
            continue;
        }

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
