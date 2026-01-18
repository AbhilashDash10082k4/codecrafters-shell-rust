use std::{process::Command};

use crate::{commands::command::UserInput, utils::path::find_executable};

//take arg -find name to attach to PATH env var and then execute the file and exit succesfully
pub fn handle(user_input: &UserInput) -> bool {
    /*M1- capturing command line arguments
        let cli_args: Vec<String> = env::args().collect();
    */
    /*M2*/
    let cmnd_arr: Vec<&str> = user_input.raw.split(' ').collect();
    let program_name = cmnd_arr[0];
    let user_args = &cmnd_arr[1..];
    
    let executable_file_path = match find_executable(program_name) {
        Some(p) => p,
        None => {
            eprintln!("{}: command not found", program_name);
            return false;
        }
    };

    match Command::new(&executable_file_path)
        .arg(program_name)
        .args(user_args)
        .output()
    {
        Ok(output) => {
            print!("{}", String::from_utf8_lossy(&output.stdout));
            if !output.stderr.is_empty() {
                eprint!("{}", String::from_utf8_lossy(&output.stderr));
            }
            println!("Program was passed {} args (including program name).", user_args.len());
            return true;
        }
        Err(e) => {
            eprintln!("Error executing {}: {}", program_name, e);
            return false;
        }
    }
}
