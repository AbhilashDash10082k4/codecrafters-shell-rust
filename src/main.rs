#![allow(unused_imports)]
use std::io::{self, Write};
fn main() {
    /* Collecting user arguments in a vec-
    Reference -chapter 12 I/O Project
    let command:Vec<String> = env::args().collect();
    dbg!(&command);
    env::args() = iterator, .collect() -turns the iterator into a collection

    dbg- used with #[derive(Debug)] -used to print the types which do not implement Display trait by default and Debug(a trait) is used to print the all the values of the type -used with {:?} or {:#?} in println!()

    debug = macro, Debug = trait

    prints the line no. and the val where the dbg! is used and takes ownership of the val (println! takes reference) and returns it back to the expression*, else completely takes the ownership of the variable passed into it
    */
    fn exit(command: &str) -> bool {
        if command.trim() == "exit" {
            return true;
        } else {
            return false;
        }
    }
    fn echo(command: &String) -> (&str, &str) {
        let processing = command.as_bytes().iter().enumerate();
        let mut space_idx = None;
        for (i, &item) in processing {
            if item == b' ' {
                space_idx = Some(i);
                break;
            }
        }
        match space_idx {
            Some(i) => {
                let cmd = &command[..i];
                let args = command[i + 1..].trim();
                (cmd, args)
            }
            None => (command.trim(), ""),
        }
    }
    fn type_command(command: &str) {
        let type_cmnd = command.trim().starts_with("type ");
        if !type_cmnd {
            return;
        }
        let command_to_be_printed = command.trim().strip_prefix("type ");

        let list_of_builtin_cmmnds = vec!["echo", "exit", "type"];
        let mut found = false;

        match command_to_be_printed {
            Some(x) => {
                for item in list_of_builtin_cmmnds {
                    if &x == &item {
                        println!("{} is a shell builtin", &item.trim());
                        found = true;
                        break;
                    }
                }
                if !found {
                    println!("{}: command not found", command_to_be_printed.unwrap())
                }
            }
            None => {},
        }
    }
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();

        //taking the user i/p
        io::stdin().read_line(&mut command).unwrap();

        //parsing the command
        if exit(&command) {
            break;
        }
        let (echo, args) = echo(&command);
        if echo == "echo" {
            println!("{}", args);
            continue;
        }

        if command.trim().contains("type ") {
            type_command(&command);
            continue;
        }
        println!("{}: command not found", &command.trim());
    }
}
