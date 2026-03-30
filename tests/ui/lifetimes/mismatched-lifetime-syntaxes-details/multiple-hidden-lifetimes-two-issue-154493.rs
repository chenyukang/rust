//@ run-rustfix
//@ rustfix-only-machine-applicable

#![deny(mismatched_lifetime_syntaxes)]

struct Pair<'a, 'b>(&'a u8, &'b u8);

macro_rules! repeated {
    ($($pair:ident),+ ; $middle:ty) => {
        ($($pair),+, $middle, $($pair),+)
        //~^ ERROR hiding or eliding a lifetime that's named elsewhere is confusing
    };
}

fn repeated_macro<'a>(x: &'a u8) -> repeated!(Pair, Pair; &'_ u8) {
    (Pair(x, x), Pair(x, x), x, Pair(x, x), Pair(x, x))
}

fn main() {}
