// The problem in #66357 was that the call trace:
//
// - parse_fn_block_decl
// - expect_or
// - unexpected
// - expect_one_of
// - expected_one_of_not_found
// - recover_closing_delimiter
//
// ended up bubbling up `Ok(true)` to `unexpected` which then used `unreachable!()`.
// error-pattern:  expected one of

fn f() { |[](* } //~ ERROR expected one of
//~^ ERROR expected one of
//~^^ ERROR mismatched closing delimiter
//~| ERROR `main` function not found
