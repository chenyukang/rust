// Issue: https://github.com/rust-lang/rust/issues/141134
// Nested import suggestion should maintain the same visibility level

#[allow(unused_imports)]
mod a {
    #[macro_export]
    macro_rules! m {
        () => {};
    }
    pub struct S;

    mod b0 {
        pub use super::{m, S};
        //~^ ERROR unresolved import `super::m`
    }

    mod b1 {
        pub(crate) use super::{m, S};
        //~^ ERROR unresolved import `super::m`
    }

    mod b2 {
        pub(in crate::a) use super::{m, S};
        //~^ ERROR unresolved import `super::m`
    }

    mod b3 {
        pub(super) use super::{m, S};
        //~^ ERROR unresolved import `super::m`
    }

    mod b4 {
        pub(self) use super::{m, S};
        //~^ ERROR unresolved import `super::m`
    }

    mod b5 {
        use super::{m, S}; // this case should not have visibility prefix
        //~^ ERROR unresolved import `super::m`
    }
}

fn main() {}
