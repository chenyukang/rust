//@ aux-build:unreachable_pattern_nested_macro.rs

extern crate unreachable_pattern_nested_macro;

use unreachable_pattern_nested_macro::create_pats;

create_pats!();

#[deny(unreachable_patterns)]
fn demo(value: u8) -> i32 {
    match value {
        pat_a!() => 42,
        pat_b!() => 24,
        //~^ ERROR unreachable pattern
    }
}

fn main() {}
