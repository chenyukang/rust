// Test that the error message for trying to move a `&mut` out of a `static`
// mentions that `&mut T` doesn't implement `Copy`.
// Issue: https://github.com/rust-lang/rust/issues/142772

static DANGLING: &mut () = unsafe { &mut *(1 as *mut ()) };
//~^ NOTE this `static` cannot be borrowed as mutable

const WUT: &mut () = DANGLING;
//~^ ERROR cannot borrow `*DANGLING` as mutable, as `DANGLING` is an immutable static item
//~| NOTE cannot borrow as mutable
//~| NOTE `&'static mut ()` doesn't implement `Copy`, so it cannot be copied from the static

fn main() {}
