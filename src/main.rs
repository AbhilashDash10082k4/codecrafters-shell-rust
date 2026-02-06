mod builtins;
mod commands;
mod shell;
mod utils;
use shell::repl::start;
fn main() {
   start();
}
