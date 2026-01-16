mod shell;
mod builtins;
mod commands;
mod utils;
use shell::repl::start;
fn main() {
    start();
}
