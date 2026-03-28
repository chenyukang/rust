//@ check-pass
#![deny(unused_assignments)]

fn used_via_raw_ptr_direct_assignment() {
    let mut a = 0;
    let a_ptr = &raw mut a;
    a = 3;
    println!("a: {}", unsafe { *a_ptr });
}

fn used_via_mut_cast_direct_assignment() {
    let mut a = 0;
    let a_ptr: *mut i32 = &mut a as *mut _;
    a = 3;
    println!("a: {}", unsafe { *a_ptr });
}

fn main() {
    used_via_raw_ptr_direct_assignment();
    used_via_mut_cast_direct_assignment();
}
