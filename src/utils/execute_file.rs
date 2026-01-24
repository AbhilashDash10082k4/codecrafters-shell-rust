use crate::{
    utils::{childprocess_execution, path::find_executable},
};

//take arg -find name to attach to PATH env var and then execute the file and exit succesfully
pub fn handle(user_input: &Vec<String>) -> bool {
    /*M1- capturing command line arguments
        let cli_args: Vec<String> = env::args().collect();
    */
    /*M2
    split_whitespace used instead of split(' ') to handle multiple whitespaces
    */
    let cmnd_arr = user_input;
    if cmnd_arr.is_empty() {
        return false;
    }
    let program_name = &cmnd_arr[0];
    match find_executable(&program_name) {
        Some(p) => {
            childprocess_execution::handle(p, &cmnd_arr);
        },
        None => {
            return false;
        }
    };    
    true
}
