use crate::utils::{
   childprocess_execution,
   path::{find_executable, is_executable},
};

//take arg -find name to attach to PATH env var and then execute the file and exit succesfully
pub fn handle(user_input: &Vec<String>) -> bool {
   if user_input.is_empty() {
      return false;
   }
   let program_name = &user_input[0];
   let paths = find_executable(program_name);
   if let Some(path) = paths {
      if is_executable(&path) {
         childprocess_execution::handle(program_name, &user_input);
         return true;
      }
   }
   false
}
