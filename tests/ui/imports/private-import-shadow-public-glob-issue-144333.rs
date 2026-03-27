#![allow(dead_code, unused_imports)]

//@ check-pass
//@ edition: 2024

mod external {
    pub fn date_range() {}
    pub fn time_range() {}
}

mod dsl {
    use crate::prelude::{date_range, time_range};

    pub mod functions {
        mod range {
            pub fn date_range() {}
            pub fn time_range() {}
        }

        pub use range::*;
    }

    pub use functions::*;
}

mod prelude {
    pub use crate::dsl::*;
    pub(crate) use crate::external::{date_range, time_range};
}

use dsl::{date_range, time_range};

fn main() {
    date_range();
    time_range();
}
