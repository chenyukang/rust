//@ run-rustfix

// Suggest all edits needed to pass an explicitly shared borrow where a mutable borrow is expected.

#![allow(dead_code, unused_mut, unused_parens)]

fn takes_mut(_: &mut String) {}

fn immutable_binding() {
    let value = String::new();
    //~^ HELP consider changing the borrow and binding to be mutable
    takes_mut(&value);
    //~^ ERROR mismatched types
}

fn mutable_binding() {
    let mut value = String::new();
    takes_mut(&value);
    //~^ ERROR mismatched types
    //~| HELP consider changing this borrow's mutability
}

fn immutable_parameter(value: String) {
    //~^ HELP consider changing the borrow and binding to be mutable
    takes_mut(&value);
    //~^ ERROR mismatched types
}

fn parenthesized_borrow() {
    let value = String::new();
    //~^ HELP consider changing the borrow and binding to be mutable
    takes_mut((&value));
    //~^ ERROR mismatched types
}

fn comment_after_borrow() {
    let value = String::new();
    //~^ HELP consider changing the borrow and binding to be mutable
    takes_mut(&/* keep */ value);
    //~^ ERROR mismatched types
}

fn main() {}
