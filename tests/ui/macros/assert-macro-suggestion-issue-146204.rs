//@ edition: 2024
// Regression test for https://github.com/rust-lang/rust/issues/146204
// When using assert_eq!/assert_ne! with mismatched types, the suggestion should not
// point to the macro definition, but should either point to user code or
// be suppressed entirely.

fn main() {
    let buf = [0_u8; 4];
    assert_ne!(buf, b"----");
    //~^ ERROR can't compare `[u8; 4]` with `&[u8; 4]`

    assert_eq!(buf, b"----");
    //~^ ERROR can't compare `[u8; 4]` with `&[u8; 4]`
}
