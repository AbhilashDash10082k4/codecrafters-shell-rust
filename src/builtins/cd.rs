use crate::commands::command::UserInput;
use std::fs::canonicalize;
use std::{env, path::Path};
/*boolean contract-
command recognised and operation succeeded-true
command recognised and operation failed -true
command is not recognised -false*/
/*change the curr_dir given by pwd
-handle absolute paths*/
pub fn handle(cmnd: &UserInput) -> bool {
    /*user_ip = path to redirect to*/
    let user_ip: Vec<&str> = cmnd.raw.trim().split_whitespace().collect();

    /*builtin cd*/
    let cd = user_ip[0];
    if user_ip.is_empty() || !matches!(cd, "cd") {
        return false;
    }
    let ip_path = user_ip[1];
    
    /*stage 15- ~ command */
    if matches!(ip_path, "~") {
        let Some(user_home_dir) = env::home_dir() else {
            println!("cd: {}: No such file or directory", ip_path);
            return true;
        };
        env::set_current_dir(user_home_dir);
    }

    /* Parsing i/p string into a Path -handled cases , so return true*/
    let gen_path = match canonicalize(Path::new(ip_path)) {
        Ok(p) => p,
        _ => {
            println!("cd: {}: No such file or directory", ip_path);
            return true;
        }
    };
    /* if !path_to_change.starts_with("/") {
    //     return false;
    // }*/

    /*validity is checked, now set the value of path_to_change to current_dir*/
    /*earlier -
    let child = Command::new(cd).current_dir(path_to_change).spawn(); -this is wrong for following reasons -
    - this spawns a child process which terminates after changing the directory of the process but the parent dir of the shell remains same
    -child processes never affect the parent process
    */
    if let Ok(_new_curr_pathl) = env::set_current_dir(&gen_path) {
        return true;
    } else {
        println!("cd: {}: No such file or directory", gen_path.display());
        return true;
    }
}
