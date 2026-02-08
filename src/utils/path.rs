use std::{
   env,
   fs::read_dir,
   path::{Path, PathBuf},
};
/*2scenarios
-user types full cmnd - take teh cmnd, take the PATH, chk the existence of that cmnd/file in that dir by iterating over all dirs from PATH separately and patching them up with the cmnd separately, chk for permissions
-user types half cmnd -on 2nd TAB press -> */
/*Search PATH
could have implemented using PathBuf::components*/
/*std::path- provides Path and PathBuf wrappers around OsStr and OsString (strings provided directly by the system, their encoding is unknown)
std::path::Path - let to work with the files in both Unix and Windows
components- str b/w path separators based on OS
Path and PathBuf are easily converted to OsStr adn OsString but not to &str and String
*/
/*Path converts str slice into path slice, join returns PathBuf
this line `let full_path = Path::new(dir).join(cmnd)` takes cmnd as file, chks every dir for the presence of that file and if true then checks for the executability of that file*/
pub fn find_executable(complete_cmd: &str) -> Option<PathBuf> {
   /*? -same as match statement. Calls from fn from From trait which converts the possible Error into the form of error returned by the fn*/
   let cmnd = complete_cmd.trim();
   let path_to_chk = env::var("PATH").unwrap_or_else(|e| format!("{e}"));
   let entries = path_to_chk.split(":");
   for dir in entries {
      let full_path = Path::new(dir).join(cmnd);

      if !full_path.is_file() {
         /*condition for incomplete command- list all the files and match*/
         continue;
      }
      if is_executable(&full_path) {
         return Some(full_path);
      }
   }
   None
}
pub fn find_completions(incomplete_cmd: &str) -> Vec<PathBuf> {
   /*returns list of paths that whose file names start with the incomplete cmnd*/
   let path_to_chk = env::var("PATH").unwrap_or_else(|e| format!("{e}"));
   let entries = path_to_chk.split(":");
   let mut vec_path = vec![]; //list of paths
   let cmnd = incomplete_cmd.trim();

   for dir in entries {
      let list_files_in_dir_to_match = read_dir(dir);
      let Ok(entries) = list_files_in_dir_to_match else {
         continue;
      };
      /*map- is lazy-> only plans the iteration and never really executes it
      .collect()-> consumes it*/
      entries.into_iter().for_each(|f| {
         if let Ok(dir_entry) = f {
            let p = dir_entry.path(); //path of files present in the curerntly scanned dir
            if p.is_file() && is_executable(&p) {
               if let Some(file_path) = p.file_name() {
                  match file_path.to_str() {
                     Some(f) => {
                        if f.starts_with(cmnd) {
                           vec_path.push(p)
                        }
                     }
                     _ => {}
                  }
               }
            }
         }
      });
   }
   vec_path.sort();
   vec_path
}

pub fn is_executable(full_path: &PathBuf) -> bool {
   #[cfg(unix)]
   {
      use std::os::unix::fs::PermissionsExt;
      if let Ok(metadata) = full_path.metadata() {
         if metadata.permissions().mode() & 0o111 != 0 {
            return true;
         }
      }
      return false;
   }
   #[cfg(windows)]
   {
      if matches!(
         full_path.extension().and_then(|e| e.to_str()),
         Some("exe" | "cmd" | "bat" | "com")
      ) {
         return true;
      }
      return false;
   }
}
/* str::matches -returns an iterator over disjoint parts that matche over patterns
std::matches (macro) -matches the 1st provided arg with the second
extension() -> Option<&OsStr> extension of an existing file
and_then() -> works on Option and Result. If Option/Result exists and is valid then do the work given as argument(could be a closure or a fn). If no valid Option/Result, then fail/crash. Prevents nested match
OsStr::to_str -returns Option<&str> -again an Option because some extensions may not be valid UTF-8
*/
