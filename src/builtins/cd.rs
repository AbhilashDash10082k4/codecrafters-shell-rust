use crate::commands::command::UserInput;
use std::{path::Path, process::Command};

/*change the curr_dir given by pwd
-handle absolute paths*/
pub fn handle(cmnd: &UserInput) -> bool {
    /*user_ip = path to redirect to*/
    let user_ip: Vec<&str> = cmnd.raw.trim().split_whitespace().collect();

    /*builtin cd*/
    let cd = user_ip[0];
    if !matches!(cd, "cd") {
        return false;
    }

    /* Parsing i/p string into a Path */
    let path_to_change = Path::new(user_ip[1]);
    if !path_to_change.starts_with("/") {
        return false;
    }

    /*validity is checked, now set the value of path_to_change to current_dir*/
    let child = Command::new(cd).current_dir(path_to_change).spawn();

    match child {
        Ok(mut c) => {
            let _ = c.wait();
            return true;
        }
        _ => {
            println!("cd: {}: No such file or directory", path_to_change.display());
            return false;
        }
    }
}
