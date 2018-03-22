//! read contains the Reader struct and its associated helpers and data
//! structures.
//!
//! https://doc.rust-lang.org/regex/regex/index.html
extern crate regex;

use self::regex::Regex;
use std::collections::HashMap;

use super::types::Symbol;

lazy_static! {
    /// The special quote characters that need additional parsing at the reader level
    pub static ref QUOTES: HashMap<&'static str, Symbol> = map!{
        "'" => Symbol::new("quote".to_string()),
        "`" => Symbol::new("quasiquote".to_string()),
        "~" => Symbol::new("unquote".to_string()),
        "~@" => Symbol::new("unquote-splicing".to_string())
    };
}

// A tag for binding input to a specific token
struct Tag {
    name:  &'static str,
    regex: Regex,
}

struct Token {
    tag:  String,
    text: String,
}

/// A Reader takes string input,either directly or from a file and produces
/// internal data forms.
pub struct Reader {
    tags:   Vec<Tag>,
    input:  String,
    ix:     i64,
    tokens: Vec<Token>,
}

impl Reader {
    pub fn new() -> Reader {
        let tags = Vec::new();
        let input = String::new();
        let ix = 0;
        let tokens = Vec::new();
        Reader { tags, input, ix, tokens }
    }

    /// read is just a stub for now that returns its argument
    pub fn read(&self, form: String) -> String {
        return form;
    }
}
