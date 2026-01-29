//@ edition:2024
// Test that using a private function from a glob import gives a better error message

mod inner {
    fn calculate() -> usize {
        //~^ NOTE function `crate::inner::calculate` exists but is inaccessible
        //~| NOTE not accessible
        1 + 2
    }

    pub fn public_fn() -> usize {
        42
    }
}

use inner::*;

fn main() {
    calculate();
    //~^ ERROR cannot find function `calculate` in this scope
    //~| NOTE not found in this scope
    //~| NOTE the item `calculate` is private and cannot be accessed through the glob import
    public_fn(); // this should work fine
}
