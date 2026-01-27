use std::fs::canonicalize;
use std::{env, path::Path};
/*boolean contract-
command recognised and operation succeeded-true
command recognised and operation failed -true
command is not recognised -false*/
/*change the curr_dir given by pwd
-handle absolute paths*/
pub fn handle(cmnd: &Vec<String>) -> bool {
    /*user_ip = path to redirect to*/
    let user_ip = cmnd;

    /*builtin cd*/
    let cd = &user_ip[0];
    if user_ip.is_empty() || cd != &String::from("cd") {
        return false;
    }
    let ip_path = &user_ip[1];

    /*stage 15- ~ command */
    if ip_path == &String::from("~") {
        let user_home_dir = env::var("HOME");
        match user_home_dir {
            Ok(p) => {
                let _ = env::set_current_dir(p);
                return true;
            }
            _ => {
                println!("cd: {}: No such file or directory", ip_path);
                return true;
            }
        }
    }

    /* Parsing i/p string into a Path -handled cases , so return true*/
    let gen_path = match canonicalize(Path::new(&ip_path)) {
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
    -irrefutable pattern -matches all the cases a pattern represents
    -refutable pattern -some values of a pattern are not matched*/
    if let Ok(_new_curr_pathl) = env::set_current_dir(&gen_path) {
        return true;
    } else {
        println!("cd: {}: No such file or directory", gen_path.display());
        return true;
    }
}
