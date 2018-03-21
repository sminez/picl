//! PICL REPL
//! This is the main Read-Eval-Print-Loop for the PICL interpretor.
//! For the interpretor class itself see intrepretor.rs

use std::io::{self, Write};


/// Read a string from input.
pub fn read() -> String {
    print!("\nλ > ");
    io::stdout().flush().unwrap();

    // Read the user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect(
        "Failed to read from stdIn",
    );
    return input
}


/// Evaluate a string.
pub fn eval(string: String) -> String {
    return string
}


/// Print a result after interpretation.
pub fn print(string: String) {
    print!("{}", string)
}


/// Run the repl
pub fn repl() -> Result<(), &'static str> {
    loop {
        print(eval(read()))
    }
}
