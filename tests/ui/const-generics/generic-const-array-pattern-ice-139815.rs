//@ compile-flags: --crate-type=lib

#![feature(generic_const_exprs)]
//~^ WARN the feature `generic_const_exprs` is incomplete

fn is_123<const N: usize>(
    x: [u32; {
        //~^ ERROR overly complex generic constant
        N + 1;
        5
    }],
) -> bool {
    match x {
        [1, 2] => true,
        _ => false,
    }
}
