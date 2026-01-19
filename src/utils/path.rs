use std::{env, path::{Path, PathBuf}};

pub fn find_executable(cmd: &str) -> Option<PathBuf> {
    //Read PATH
    let cmnd = cmd.trim();

    /*? -same as match statement. Calls from fn from From trait which converts the possible Error into the form of error returned by the fn*/
    let path_var = env::var("PATH").ok()?;
    
    /*Search PATH
    could have implemented using PathBuf::components*/
    for dir in path_var.split(':') {
        /*std::path- provides Path and PathBuf wrappers around OsStr and OsString (strings provided directly by the system, their encoding is unknown)
        std::path::Path - let to work with the files in both Unix and Windows
        components- str b/w path separators based on OS
        Path and PathBuf are easily converted to OsStr adn OsString but not to &str and String
        */
        /*Path converts str slice into path slice, join returns PathBuf
        this line `let full_path = Path::new(dir).join(cmnd)` takes cmnd as file, chks every dir for the presence of that file and if true then checks for the executability of that file*/
        let full_path = Path::new(dir).join(cmnd);

        if !full_path.is_file() {
            continue;
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = full_path.metadata() {
                if metadata.permissions().mode() & 0o111 != 0 {
                    return Some(full_path);
                }
            }
        }
        /*cfg -conditional compilation*/
        #[cfg(windows)]
        {
            /* str::matches -returns an iterator over disjoint parts that matche over patterns
            std::matches (macro) -matches the 1st provided arg with the second
            extension() -> Option<&OsStr> extension of an existing file
            and_then() -> works on Option and Result. If Option/Result exists and is valid then do the work given as argument(could be a closure or a fn). If no valid Option/Result, then fail/crash. Prevents nested match
            OsStr::to_str -returns Option<&str> -again an Option because some extensions may not be valid UTF-8
            */
            if matches!(
                full_path.extension().and_then(|e| e.to_str()),
                Some("exe" | "cmd" | "bat" | "com")
            ) {
                return Some(full_path);
            }
        }
    }
    None
}