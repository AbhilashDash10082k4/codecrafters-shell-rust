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
    if args.ends_with("\t") {
        for c in builtins {
            if c.starts_with(prefix) {
                print!("{c} ");
                io::stdout().flush().unwrap();
                return true;
            }
        }
    }
    false
}