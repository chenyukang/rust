//@ check-fail
#![deny(unused_assignments)]

struct S {
    value: u32,
}

fn used_via_raw_ptr() {
    let mut s = S { value: 0 };
    let ptr = &raw mut s;
    s.value = 42;
    println!("{}", unsafe { (*ptr).value });
}

fn raw_borrow_alone_is_not_enough() {
    let mut s = S { value: 0 };
    let _ptr = &raw mut s;
    s.value = 42; //~ ERROR value assigned to `s` is never read
}

fn main() {
    used_via_raw_ptr();
    raw_borrow_alone_is_not_enough();
}
