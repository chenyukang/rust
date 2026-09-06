//! Real macro expansions remain subject to the macro-export lint, including when their output
//! carries inert attributes or an attribute macro returns its input unchanged.

//@ proc-macro: proc_macro_def.rs
//@ edition: 2024

use proc_macro_def::attr_identity;

#[attr_identity]
#[macro_export]
macro_rules! exported {
    () => {};
}

#[attr_identity]
#[rustfmt::skip]
#[macro_export]
macro_rules! attributed_export {
    () => {};
}

macro_rules! define_export {
    () => {
        #[rustfmt::skip]
        #[macro_export]
        macro_rules! generated_export {
            () => {};
        }
    };
}

define_export!();

fn main() {
    crate::exported!();
    //~^ ERROR macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
    //~| WARN this was previously accepted by the compiler but is being phased out
    crate::attributed_export!();
    //~^ ERROR macro-expanded `macro_export` macros from the current crate cannot
    //~| WARN this was previously accepted
    crate::generated_export!();
    //~^ ERROR macro-expanded `macro_export` macros from the current crate cannot
    //~| WARN this was previously accepted
}
