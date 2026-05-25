use std::io::Write;

pub fn handle(cmd: &[String], out:&mut impl Write) -> bool {
   let args = cmd;
   if args.is_empty() || args[0] != "echo" {
      return false;
   }
   if args.len() > 1 {
      let _ = writeln!(out,"{}", &args[1..].join(" "));
   }
   true
}
