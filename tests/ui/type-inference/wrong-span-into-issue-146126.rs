//@ reference: https://github.com/rust-lang/rust/issues/146126
// Regression test for #146126:
// rustc was flagging the wrong line for an ambiguous `.into` type inference error.
// The error should point to the `.into()` call, not to the field access `x.f`.

struct Foo {
    f: Option<i32>,
}

fn main() {
    let v = vec![Foo { f: Some(1) }];
    if let Some(x) = v.get(0_i16.into()) {
        //~^ ERROR type annotations needed
        //~| HELP try using a fully qualified path
        if let Some(f) = x.f {
            println!("{}", f);
        } else {
            println!("not present");
        }
    } else {
        println!("bad index");
    }
}
