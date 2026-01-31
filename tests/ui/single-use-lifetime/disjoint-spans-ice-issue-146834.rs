//@ compile-flags: -Wsingle-use-lifetimes
// Regression test for issue #146834: ICE when single-use-lifetimes lint
// generates overlapping spans due to malformed attributes

#![core::contracts::ensures]
//~^ ERROR use of unstable library feature `contracts`
//~| ERROR inner macro attributes are unstable
//~| ERROR missing lifetime specifiers
//~| ERROR `#[prelude_import]` is for use by rustc only
//~| WARNING lifetime parameter `'a` only used once
//~| WARNING lifetime parameter `'b` only used once
//~| ERROR expected a `Fn(&_)` closure, found `()`

fn f4_<'a, 'b>(a: &'a i32, b: &'b i32) -> (&i32, &i32) {
    loop {}
}

fn main() {}
