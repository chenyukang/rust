//! Inert attributes must not hide an ambiguity with a glob-imported macro.
//! The export appears after the call has already resolved to the glob import.

//@ edition: 2024

#![allow(unused_imports, macro_expanded_macro_exports_accessed_by_absolute_paths)]

mod fallback {
    macro_rules! pick {
        () => {
            const VALUE: u8 = 1;
        };
    }
    pub(crate) use pick;
}

use fallback::*;

crate::pick!();
//~^ ERROR `pick` is ambiguous

mod definitions {
    #[rustfmt::skip]
    #[macro_export]
    macro_rules! pick {
        () => {
            const VALUE: u8 = 2;
        };
    }
}

fn main() {
    println!("{VALUE}");
}
