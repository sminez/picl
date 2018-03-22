extern crate picl;

use picl::repl::Repl;

fn main() {
    let repl = Repl::new();
    repl.repl()
}
