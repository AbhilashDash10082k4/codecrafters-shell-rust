#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
fn main() {
    // TODO: Uncomment the code below to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    /* Collecting user arguments in a vec-
    env::args() = iterator, .collect() -turns the iterator into a collection
    
    dbg- used with #[derive(Debug)] -used to print the types which do not implement Display trait by default and Debug(a trait) is used to print the all the values of the type -used with {:?} or {:#?} in println!()

    debug = macro, Debug = trait

    prints the line no. and the val where the dbg! is used and takes ownership of the val (println! takes reference) and returns it back to the expression*, else completely takes the ownership of the variable passed into it
    */
    let command:Vec<String> = env::args().collect();
    dbg!(&command);

    //taking the command in var
    let query = &command[1];

    println!("{}: command not found", query.trim())

}
