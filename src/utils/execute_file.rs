use crate::{
    utils::{childprocess_execution, path::find_executable},
};

//take arg -find name to attach to PATH env var and then execute the file and exit succesfully
pub fn handle(user_input: &Vec<String>) -> bool {
    let cmnd_arr = user_input;
    if cmnd_arr.is_empty() {
        return false;
    }
    let program_name = &cmnd_arr[0];
    let p = match find_executable(&program_name) {
        Some(p) => p,
        None => {
            return false;
        }
    };
    childprocess_execution::handle(p, &cmnd_arr);
    true
}
