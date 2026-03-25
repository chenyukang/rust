//@ edition: 2024
//@ check-pass
//@ run-rustfix

#![warn(unused_braces)]

fn consume<T>(_: T) {}

fn main() {
    consume({ 7 });
    //~^ WARN unnecessary braces
}
