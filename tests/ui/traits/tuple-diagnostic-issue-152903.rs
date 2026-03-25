//@ edition: 2024

use std::fmt::Debug;

fn testing<T: Debug>(_t: T) {}

struct Foo;

fn main() {
    testing((1, Foo));
    //~^ ERROR `Foo` doesn't implement `Debug`
}
