use crate::commands::command::UserInput;
use std::{env, error::Error, path::PathBuf};
/*print the current directory
type the pwd cmnd in shell -give it to OS -print the current dir
*/
pub fn handle(cmnd: &UserInput) -> bool{
    //i/p taken in form of argument
    let user_ip = cmnd.raw.trim();
    //match command with pwd and then send this to OS to fetch the curr dir
    if user_ip == "pwd" {
        let Ok(curr_dir) = env::current_dir() else {
            return false;
        };
        println!("{}", curr_dir.display());
        return true;
    } else {
        return true;
    }
    
}