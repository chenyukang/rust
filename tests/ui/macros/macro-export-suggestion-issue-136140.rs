// Regression test for issue #136140
// The suggestion to add `use crate::nothing;` for a macro-export macro
// defined in the same crate is invalid when the macro has an attribute
// (like `#[rustfmt::skip]`) because it leads to another error:
// "macro-expanded `macro_export` macros from the current crate cannot be
// referred to by absolute paths"

fn main() {}

mod a {
    #[rustfmt::skip]
    #[macro_export]
    macro_rules! nothing {
        () => {};
    }
}

mod b {
    fn foo() {
        nothing!();
        //~^ ERROR cannot find macro `nothing` in this scope
    }
}
