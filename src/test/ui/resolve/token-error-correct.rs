
// ignore-tidy-trailing-newlines
// Test that we do some basic error correction in the tokeniser.
// error-pattern: expected one of

fn main() {
    foo(bar(; //~ ERROR: expected expression
}
//~^ ERROR: mismatched closing delimiter: `}`

fn foo(_: usize) {} //~ ERROR: expected one of