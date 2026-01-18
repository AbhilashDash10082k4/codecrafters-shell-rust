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

    let executable_path = match find_executable(program_name) {
        Some(p) => p,
        None => {
            eprintln!("{}: command not found", program_name);
            return false;
        }
    };

    let mut child =match Command::new(&executable_path)
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
