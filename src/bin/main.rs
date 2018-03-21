extern crate picl;

use std::process;
use picl::repl;


fn main() {
    if let Err(e) = repl::repl() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
