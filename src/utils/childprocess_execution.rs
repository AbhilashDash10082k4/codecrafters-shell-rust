use std::{path::Path, process::{Command, Stdio}};

pub fn handle(cmnd: &Vec<String>) {
    if let Ok(mut child) = Command::new(Path::new(&cmnd[0]))
        .args(&cmnd[1..])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
    {
        let _ = child.wait();
    }
}
