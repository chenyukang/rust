//! Regression test for https://github.com/rust-lang/rust/issues/141742.
//! An unconstrained generic enum pattern should suggest where to specify its type argument.

fn main() {
    if let Some(whatever) = todo!() {
        //~^ ERROR type annotations needed
        todo!();
    } else {
        todo!();
    }
}
