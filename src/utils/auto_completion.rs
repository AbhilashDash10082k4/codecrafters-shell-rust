use std::io::{self, Write};

pub fn handle(args: &str) -> bool{
    if args.is_empty() {
        return false;
    }
    /*jab 3 letter baad user \t type kare, tab woh apne aap complete ho jae*/
    let builtins = ["echo", "exit"];
    let prefix = args
        .trim_end_matches('\n')
        .trim_end_matches('\t');
    if !args.ends_with('\t') {
        return false;
    }
        for c in builtins {
            if c.starts_with(prefix) {
                /*this does not overwrites the "ech" or "exi"
                todo- take cursor to start -> overwrite old i/p (ech, exi) -> write teh completed o/p*/
                print!("\r\x1b[2K$ {c} ");
                /*flush -> stateful -> sends bytes to terminal -> depends on cursor position*/
                io::stdout().flush().unwrap();
                return true;
            }
        
    }
    false
}