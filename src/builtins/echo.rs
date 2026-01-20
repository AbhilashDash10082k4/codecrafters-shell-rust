use crate::{commands::command::UserInput, utils::cmnd_parser};
pub fn handle(cmd: &UserInput) ->bool {
    /*as_bytes- byte slice (vector of bytes) of a string
    -from_utf8- converts byte arr into string
    -into_bytes-same as as_bytes -consumes the string
    -iter -iterates over &T
    -into_mut_iter -iterates over &mut T
    -into_iter -iterates over T
    */
    /*V1-
    let processing = ip_str.as_bytes().iter().enumerate();
    let mut space_idx = None;
    for (i, &item) in processing {
        if item == b' ' {
            space_idx = Some(i);
            break;
        }
    }
    match space_idx {
        Some(i) => {
            let cmd = &ip_str[..i];
            let args = ip_str[i + 1..].trim();
            (cmd, args)
        }
        None => (ip_str.trim(), ""),
    }*/
    
    
    let args = cmnd_parser::handle(cmd);
    if args[0] == "echo" {
        println!("{:#?}",&args[1..]);
    }
    true
}