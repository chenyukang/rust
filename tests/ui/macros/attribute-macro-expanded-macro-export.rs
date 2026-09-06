//! A real attribute macro still makes a `macro_export` definition macro-expanded, even if the
//! attribute returns its input unchanged.

//@ proc-macro: proc_macro_def.rs
//@ edition: 2024

use proc_macro_def::attr_identity;

#[attr_identity]
#[macro_export]
macro_rules! exported {
    () => {};
}

fn main() {
    crate::exported!();
    //~^ ERROR macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
    //~| WARN this was previously accepted by the compiler but is being phased out
}
