//@ run-rustfix
//@ compile-flags: --edition 2021
#![deny(unused_imports)]
#![allow(dead_code)]

fn test0() {
    // Test remove FlatUnused
    use std::convert::TryFrom;
    //~^ ERROR redundant import
    let _ = u32::try_from(5i32);
}

fn test1() {
    // Test remove NestedFullUnused
    use std::convert::{TryFrom, TryInto};
    //~^ ERROR redundant imports

    let _ = u32::try_from(5i32);
    let _a: i32 = u32::try_into(5u32).unwrap();
}

fn test2() {
    // Test remove both redundant and unused
    use std::convert::{AsMut, Into};
    //~^ ERROR unused or redundant imports: `AsMut`, `Into`

    let _a: u32 = (5u8).into();
}

fn test3() {
    // Test remove NestedPartialUnused
    use std::convert::{From, Infallible};
    //~^ ERROR unused import: `From`

    trait MyTrait {}
    impl MyTrait for fn() -> Infallible {}
}

fn main() {}
