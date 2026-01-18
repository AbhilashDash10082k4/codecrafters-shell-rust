#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::{
    env, fs,
    path::{MAIN_SEPARATOR_STR, Path},
};

impl Command {
    
    

    // 4️⃣ Not found
    println!("{command_to_be_printed}: not found");
}

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
