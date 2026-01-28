use std::{
    fs::File,
    process::{Command, Stdio},
};
/*Work of this block -take an executable file and handles it to the terminal -basically start a program
- Command::new -prepares the file to be executed , takes in the name of the program and not the entire file path
- args -takes the extra arguments provided by the user
- .stdin(Stdio::inherit) -is a child program and takes i/p from keyboard as same as shell
Diff b.w child program and shell-
Shell -parent, is this current program and manager of diff OS processes, runs other codes(child process), takes the keyboard i/p
Child program -ran by shell, temporary, exist only when the code is running, executed by OS, is the process created by OS to run a code
stdin (i/p from keyboard), stdout (o/p to screen), stderr(err to screen) -pipes
.Stdio::inherit() -makes the shell and child program share the same terminal/ o/p screen
Shell controls who runs and child program controls what runs
Command -tool to run other programs, used by Rust to command directly to OS
.spawn -finally runs the program
child.wait -used to hold the shell untill the program stops
program_name != find_executable(program_name)
*/

pub fn handle(program_name: &str, args: &Vec<String>) {
    /*earlier &cmnd[0] worked as the executables didnt have spaces. But now, absolute paths are needed to pass inorder to make the execution successful */
    if args.is_empty() {
        return;
    }
    let mut file_name = None;
    // let op_idx = args.iter().position(|r| r == ">").expect("Err");
    let mut cmnd_args = Vec::new();
    let mut i = 1;
    let mut output_redirect_char: Option<&str> = None;

    /*stage24 -stderr redirect -> decide where to put the results before even spawning/running the execution */

    while i < args.len() {
        if &args[i] == ">" || &args[i] == "1>" || &args[i] == "2>" || &args[i] == ">>" {
            output_redirect_char = Some(&args[i]);
            if i + 1 < args.len() {
                file_name = Some(&args[i + 1]);
            }
            /*exit the loop since the redirection file is found coz anything after the redirection file name is ignored*/
            break;
        }
        cmnd_args.push(&args[i]);
        i += 1;
    }
    /*Cannot do directly this -Command::new(program_name)..args().stdin().stderr() becoz  creates a temporary val while being used*/
    let mut child = Command::new(program_name);
    child.args(cmnd_args);
    child.stdin(Stdio::inherit());

    if let Some(f) = file_name {
        if output_redirect_char == Some(">")
            || output_redirect_char == Some("1>")
        {
            /*stage24- rediretion is applied before command execution- for both external and builtin execution
            correct order of shell -> parser -> detect redirection-> setup stdout -> cmnd execution
            here, redirection is done on the basis of character*/
            child.stdout(Stdio::from(File::create(f).expect("Err")));
        } else if output_redirect_char == Some(">>") {
            /*stage25 */
            let append_content = File::options().append(true).create(true).open(f).ok();
            match append_content {
                Some(c) => {
                    child.stdout(Stdio::from(c));
                }
                _ => return,
            }
        } else {
            child.stdout(Stdio::inherit());
        }
        if output_redirect_char == Some("2>") {
            child.stderr(Stdio::from(File::create(f).expect("Err")));
        } else {
            child.stderr(Stdio::inherit());
        }
    }

    if let Ok(mut c) = child.spawn() {
        let _ = c.wait();
    }
}
/*stage23 ref -
let mut buffer = File::create("foo.txt")?;
buffer.write_all(b"some bytes")?;
io::stdout().write_all(b"hello world")?;

let output = Command::new("/bin/cat")
    .arg("file.txt")
    .output()?;

println!("status: {}", output.status);
io::stdout().write_all(&output.stdout)?;
io::stderr().write_all(&output.stderr)?;

let mut child = Command::new("rev")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .expect("Failed to spawn child process");

let mut stdin = child.stdin.take().expect("Failed to open stdin");
std::thread::spawn(move || {
    stdin.write_all("Hello, world!".as_bytes()).expect("Failed to write to stdin");
});

let output = child.wait_with_output().expect("Failed to read stdout")*/
