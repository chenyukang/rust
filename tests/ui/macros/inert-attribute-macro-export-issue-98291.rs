//! Inert attributes do not make source-defined `macro_export` macros macro-expanded.

//@ check-pass

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

mod child {
    use crate::formatted;

    pub fn check() {
        let _ = formatted!("formatted");
    }
}

fn main() {
    let _ = renamed!();
    let _ = crate::exported!();
    crate::configured!();
    child::check();
}
