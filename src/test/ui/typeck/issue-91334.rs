// Regression test for the ICE described in issue #91334.

// error-pattern: expected one of
// error-pattern: mismatched closing delimiter
// error-pattern: mismatched types

#![feature(generators)]

fn f(){||yield(((){),
