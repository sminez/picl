//! PICL a LISP written in Rust

// Internal modules
pub mod builtins;
pub mod env;
pub mod eval;
pub mod interpretor;
pub mod pmatch;
pub mod read;
pub mod repl;
pub mod types;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
