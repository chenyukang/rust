//@ compile-flags: -Wsingle-use-lifetimes
//@ edition: 2024

#[core::contracts::ensures]
//~^ ERROR  use of unstable library feature `contracts` [E0658]
//~| ERROR `ensures` attribute requires an argument
fn f<'a, 'b>(a: &'a i32, b: &'b i32) -> (&i32, &i32) {
    loop {}
}

fn main() {}
