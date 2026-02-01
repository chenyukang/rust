//@ check-fail
// Test for #151876: duplicate errors for missing trait instance in struct definition

trait With {
    type Assoc;
}

struct Foo(<u32 as With>::Assoc);
//~^ ERROR the trait bound `u32: With` is not satisfied

fn use_foo(f: Foo) {
    drop(f);
}

fn main() {}
