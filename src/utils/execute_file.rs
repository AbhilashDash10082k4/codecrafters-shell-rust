use std::process::{Command, Stdio};

use crate::{commands::command::UserInput, utils::path::find_executable};

//take arg -find name to attach to PATH env var and then execute the file and exit succesfully
pub fn handle(user_input: &UserInput) -> bool {
    /*M1- capturing command line arguments
        let cli_args: Vec<String> = env::args().collect();
    */
    /*M2
    split_whitespace used instead of split(' ') to handle multiple whitespaces
    */
    let cmnd_arr: Vec<&str> = user_input.raw.split_whitespace().collect();
    if cmnd_arr.is_empty() {
        return false;
    }
    let program_name = cmnd_arr[0];
    let user_args = &cmnd_arr[1..];
    match find_executable(program_name) {
        Some(p) => p,
        None => {
            
            return false;
        }
    };

    /*Work of this block -take an executable file and handles it to the terminal -basically start a program
    - Command::new -prepares the file to be executed , takes in the name of the program and not the entire file path
    - args -takes the extra arguments provided by the user
    - .stdin(Stdio::inherit) -is a child program and takes i/p from keyboard as same as shell
    Diff b.w child program and shell-
    Shell -parent, is this current program and manager of diff OS processes, runs other codes(child process), takes the keyboard i/p
    Child program -ran by shell, temporary, exist only when the code is running, executed by OS, is the process created by OS to run a code
    stdin (i/p from keyboard), stdout (o/p to screen), stderr(err to screen) -pipes
    .Stdio::inherit() -makes the shell and child program share the same terminal/ o/p screen
    Shell controls who runs and child program controls what runs
    Command -tool to run other programs, used by Rust to command directly to OS
    .spawn -finally runs the program
    child.wait -used to hold the shell untill the program stops
    program_name != find_executable(program_name) 
    */
    let mut child = match Command::new(program_name)
        .args(user_args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error executing {program_name}: {e}");
            return false;
        }
    };
    // Wait for program to finish before showing prompt again
    let _ = child.wait();
    true
}
