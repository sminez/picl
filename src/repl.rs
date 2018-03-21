//! PICL REPL
//! This is the main Read-Eval-Print-Loop for the PICL interpretor.
//! For the interpretor class itself see intrepretor.rs

extern crate rustyline;

use self::rustyline::Editor;
use self::rustyline::error::ReadlineError;

/// Evaluate a string.
fn eval(string: String) -> String {
    return string;
}

/// Print a result after interpretation.
pub fn print(string: String) {
    println!("... {}\n", string)
}

/// Run the repl
pub fn repl() {
    let mut rl = Editor::<()>::new();
    // Don't worry if there is no history file yet
    if let Err(_) = rl.load_history("/tmp/picl-history.txt") {}

    // READ loop
    loop {
        let readline = rl.readline("λ > ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                print(eval(line));
            }

            Err(ReadlineError::Interrupted) => {
                break; // CTRL-C
            }

            Err(ReadlineError::Eof) => {
                break; // CTRL-D
            }

            // Some other readline error so report it and bail.
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("/tmp/picl-history.txt").unwrap();
}
