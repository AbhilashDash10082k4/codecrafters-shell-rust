use std::env;
/*print the current directory
type the pwd cmnd in shell -give it to OS -print the current dir
*/
pub fn handle(cmnd: &Vec<String>) -> bool {
    //i/p taken in form of argument
    let user_ip = cmnd[0].trim();
    //match command with pwd and then send this to OS to fetch the curr dir
    if user_ip == "pwd" {
        if let Ok(curr_dir) = env::current_dir() {
            println!("{}", curr_dir.display());
        }
        return true;
    }
    false
}
