#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::{
    env, fs,
    path::{MAIN_SEPARATOR_STR, Path},
};

pub struct Command {
    pub command: String,
}
impl Command {
    pub fn exit(&self) -> bool {
        if self.command.trim() == "exit" {
            return true;
        } else {
            return false;
        }
    }
    pub fn echo(&self) -> (&str, &str) {
        /*as_bytes- byte slice (vector of bytes) of a string
        -from_utf8- converts byte arr into string
        -into_bytes-same as as_bytes -consumes the string
        -iter -iterates over &T
        -into_iter -iterates over &mut T
        -into_iter -iterates over T
        */
        let processing = self.command.as_bytes().iter().enumerate();
        let mut space_idx = None;
        for (i, &item) in processing {
            if item == b' ' {
                space_idx = Some(i);
                break;
            }
        }
        match space_idx {
            Some(i) => {
                let cmd = &self.command[..i];
                let args = self.command[i + 1..].trim();
                (cmd, args)
            }
            None => (self.command.trim(), ""),
        }
    }
    pub fn type_command(&self) {
    let command_to_be_printed = match self.command.trim().strip_prefix("type ") {
        Some(c) => c.trim(),
        None => return,
    };

    let builtins = ["echo", "exit", "type"];

    // 1️⃣ Builtin check
    if builtins.contains(&command_to_be_printed) {
        println!("{command_to_be_printed} is a shell builtin");
        return;
    }

    // 2️⃣ Read PATH
    let path_var = match env::var("PATH") {
        Ok(v) => v,
        Err(_) => {
            println!("{command_to_be_printed}: not found");
            return;
        }
    };

    // 3️⃣ Search PATH
    for dir in path_var.split(':') {
        let full_path = Path::new(dir).join(command_to_be_printed);

        if !full_path.is_file() {
            continue;
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = full_path.metadata() {
                if metadata.permissions().mode() & 0o111 != 0 {
                    println!("{command_to_be_printed} is {}", full_path.display());
                    return;
                }
            }
        }

        #[cfg(windows)]
        {
            if matches!(
                full_path.extension().and_then(|e| e.to_str()),
                Some("exe" | "cmd" | "bat" | "com")
            ) {
                println!("{command_to_be_printed} is {}", full_path.display());
                return;
            }
        }
    }

    // 4️⃣ Not found
    println!("{command_to_be_printed}: not found");
}

}

// pub fn find_executable(&self) -> Option<String> {
//     //ok -converts Result<T,E> to Option<T,E>, consuming self and discarding error if any
//     let path_var = env::var("PATH").ok()?;

//     for dir in path_var.split(MAIN_SEPARATOR_STR) {
//         let full_path = Path::new(dir).join(self.command);

//         if let Ok(metadata) = fs::metadata(&full_path) {
//             let permissions = metadata.permissions();
//             if permissions.mode() & 0o111 != 0 {
//                 return Some(full_path.display().to_string());
//             }
//         }
//     }
//     None
// }

/*file creation-
let file_read: Result<T, io::Error> = File::open("a.txt");
let file_read_res = match file_read {
    Ok(file) => file,
    //error = io::Error (Struct), kind -method defined in this struct and returns io::ErrorKind enum provided by the std lib
    Err(error) => match error.kind() {
        // ErrorKind::NotFound -NotFound value/variant returned from ErrorKind
        ErrorKind::NotFound => match File::create("a.txt") {
            Ok(fc) => fc,
            Err(e) => panic!()
        },
        _ => panic!()
    }
}

//same fn by using closures ->
let file_read: Result<String, io::Error> = File::open("b.txt").unwrap_or_else(|err| {
    if err.kind() == ErrorKind::NotFound {
        File::create("b.txt").unwrap_or_else(|err| {
            panic(!("{err:?}"));
        })
    }else {
        panic!("");
    }
});

?- propagating error and success from the called fn to the calling fn
used on Result -if val is Ok(T) -> T will get returned from this and stored in the variable
if Err -> this go through from fn defined in From trait, converted from there og type to the type defined in the return type of fn and are returned from the complete fn

? is used on fns which return types Result, Option or any type that implements FromResidual

ok -converts Result to Option
ok_or- reverse of ok
*/
