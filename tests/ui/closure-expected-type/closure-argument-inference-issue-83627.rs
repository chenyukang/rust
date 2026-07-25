//@ check-pass

//! Regression test for <https://github.com/rust-lang/rust/issues/83627>.
//! An unrelated impl should not prevent inferring a closure argument from the applicable impl.

trait Foo<P> {
    fn foo(&self, p: P);
}

impl<F: FnMut(&u8) -> bool> Foo<F> for [u8] {
    fn foo(&self, _f: F) {}
}

impl Foo<u8> for [u8] {
    fn foo(&self, _c: u8) {}
}

fn main() {
    b"abc".foo(|b| match b {
        b' ' => true,
        _ => false,
    });

    b"abc".foo(|b| *b == b' ');

    b"abc".foo(|&b| b == b' ');
}

trait Coerce<P> {
    fn coerce(&self, p: P);
}

impl Coerce<fn(&u8) -> bool> for [u8] {
    fn coerce(&self, _f: fn(&u8) -> bool) {}
}

fn closure_to_fn_pointer() {
    b"abc".coerce(|b| *b == b' ');
}
