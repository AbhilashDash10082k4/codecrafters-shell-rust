use crate::commands::command::UserInput;
use std::{env, path::Path};
use std::fs::canonicalize;

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
    let gen_path = match canonicalize(Path::new(user_ip[1])) {
        Ok(p) =>p,
        _ => {
            return true;
        }
    };
    // if !path_to_change.starts_with("/") {
    //     return false;
    // }

    /*validity is checked, now set the value of path_to_change to current_dir*/
    /*earlier -
    let child = Command::new(cd).current_dir(path_to_change).spawn(); -this is wrong for following reasons -
    - this spawns a child process which terminates after changing the directory of the process but the parent dir of the shell remains same
    -child processes never affect the parent process
    */
    if let Ok(_new_curr_pathl) = env::set_current_dir(&gen_path) {
        return true;
    } else {
        println!(
            "cd: {}: No such file or directory",
            gen_path.display()
        );
        return true;
    }
    
}
