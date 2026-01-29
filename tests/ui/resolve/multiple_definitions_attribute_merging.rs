//! This test used to ICE because the `repr(packed)` attributes
//! end up on the `Dealigned` struct's attribute list, but the
//! derive didn't see that. This is now fixed by registering
//! duplicated names as `Res::Err` to avoid further errors.

#[repr(packed)]
struct Dealigned<T>(u8, T);

#[derive(PartialEq)]
#[repr(C)]
struct Dealigned<T>(u8, T);
//~^ ERROR the name `Dealigned` is defined multiple times

fn main() {}
