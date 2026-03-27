#![allow(unused_imports)]

//@ check-pass
//@ edition:2015
mod options {
    pub struct ParseOptions {}
}

mod parser {
    pub use options::*;
    // A private import in the module should not make the public glob-imported binding
    // inaccessible to downstream paths.
    #[allow(hidden_glob_reexports)]
    use ParseOptions;
}

pub use parser::ParseOptions;

fn main() {}
