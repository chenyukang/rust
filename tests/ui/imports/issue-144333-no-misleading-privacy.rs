//@ aux-crate: issue_144333_cross=issue-144333-cross.rs
//@ edition: 2024

extern crate issue_144333_cross;

use issue_144333_cross::dsl::date_range;
//~^ ERROR `date_range` is ambiguous [ambiguous_glob_imports]
//~| WARN this was previously accepted by the compiler
use issue_144333_cross::dsl::time_range;
//~^ ERROR `time_range` is ambiguous [ambiguous_glob_imports]
//~| WARN this was previously accepted by the compiler

fn main() {}
