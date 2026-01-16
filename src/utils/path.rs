use std::{env, path::{Path, PathBuf}};

pub fn find_executable(cmd: &str) -> Option<PathBuf> {
    //Read PATH
    let cmnd = cmd.trim();
    let path_var = env::var("PATH").ok()?;
     //Search PATH
    for dir in path_var.split(':') {
        let full_path = Path::new(dir).join(cmnd);

        if !full_path.is_file() {
            continue;
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = full_path.metadata() {
                if metadata.permissions().mode() & 0o111 != 0 {
                    println!("{command_to_be_printed} is {}", full_path.display());
                    return Some(full_path);
                }
            }
        }

        #[cfg(windows)]
        {
            if matches!(
                full_path.extension().and_then(|e| e.to_str()),
                Some("exe" | "cmd" | "bat" | "com")
            ) {
                println!("{cmnd} is {}", full_path.display());
                Some(full_path);
            }
        }
    }
    None
}
