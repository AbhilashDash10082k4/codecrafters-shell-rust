use crate::commands::command::Command;
pub fn handle(cmd: &Command) ->bool {
    /*as_bytes- byte slice (vector of bytes) of a string
    -from_utf8- converts byte arr into string
    -into_bytes-same as as_bytes -consumes the string
    -iter -iterates over &T
    -into_iter -iterates over &mut T
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
    
    //V2
    let (echo_cmd, arg) = cmd.name_and_args();
    if echo_cmd == "echo" {
        println!("{}",arg);
        return true;
    }
    false
}