//@ check-pass
#![warn(unused_assignments)]

fn id(value: i32) -> i32 {
    value
}

fn direct_self_use() {
    let mut x = 1;
    x = x + 1; //~ WARN value assigned to `x` is never read
    x = 3;
    let _ = x;
}

fn compound_assign() {
    let mut x = 1; //~ WARN value assigned to `x` is never read
    x += 1; //~ WARN value assigned to `x` is never read
    x = 3;
    let _ = x;
}

fn checked_division(y: i32) {
    let mut x = 4;
    x = x / y; //~ WARN value assigned to `x` is never read
    x = 3;
    let _ = x;
}

fn call_self_use() {
    let mut x = 1;
    x = id(x); //~ WARN value assigned to `x` is never read
    x = 3;
    let _ = x;
}

fn main() {
    direct_self_use();
    compound_assign();
    checked_division(2);
    call_self_use();
}
