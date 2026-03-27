#![allow(dead_code, unused_imports)]

pub mod dsl {
    pub(crate) use crate::external::*;

    pub mod functions {
        mod range {
            pub fn date_range() {}
            pub fn time_range() {}
        }

        pub use self::range::*;
    }

    pub use self::functions::*;
}

mod external {
    pub fn date_range() {}
    pub fn time_range() {}
}
