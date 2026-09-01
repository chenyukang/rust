//! Regression test for <https://github.com/rust-lang/rust/issues/160717>.
//! Hidden implementation details should not be exposed as a similar implementation.

//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver

struct Internal;

#[diagnostic::do_not_recommend]
impl From<Internal> for &str {
    fn from(_: Internal) -> Self {
        ""
    }
}

fn require_into<'a, T: Into<&'a str>>(_: T) {}

fn main() {
    require_into(String::new());
    //~^ ERROR the trait bound `&str: From<String>` is not satisfied
}
