//! Inert attributes do not make source-defined `macro_export` macros macro-expanded.

#[rustfmt::skip]
#[macro_export]
macro_rules! exported {
    () => {
        "exported"
    };
}

#[macro_export]
#[rust_analyzer::macro_style(braces)]
#[clippy::format_args]
macro_rules! formatted {
    ($($arg:tt)*) => {
        format_args!($($arg)*)
    };
}

#[cfg_attr(all(), rustfmt::skip)]
#[macro_export]
macro_rules! configured {
    () => {};
}

pub use exported as renamed;
//~^ ERROR macro-expanded `macro_export` macros from the current crate cannot
//~| WARN this was previously accepted

mod child {
    use crate::formatted;
    //~^ ERROR macro-expanded `macro_export` macros from the current crate cannot
    //~| WARN this was previously accepted

    pub fn check() {
        let _ = formatted!("formatted");
    }
}

fn main() {
    let _ = renamed!();
    let _ = crate::exported!();
    //~^ ERROR macro-expanded `macro_export` macros from the current crate cannot
    //~| WARN this was previously accepted
    crate::configured!();
    //~^ ERROR macro-expanded `macro_export` macros from the current crate cannot
    //~| WARN this was previously accepted
    child::check();
}
