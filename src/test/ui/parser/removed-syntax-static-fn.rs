struct S;

impl S {
    static fn f() {}
    //~^ ERROR expected identifier, found keyword `fn`
    //~| ERROR expected one of `!`, `+`, `->`, `::`, `;`, or `=`, found `{`
}

fn main() {}
