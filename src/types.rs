//! Internal data types for use within PICL

/// A LISP Symbol, distinct from strings.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Symbol {
    name: String,
}

impl Symbol {
    /// Create a new Symbol from a String.
    pub fn new(name: String) -> Symbol {
        return Symbol { name };
    }
}
