//@ run-rustfix
//@ rustfix-only-machine-applicable

#![deny(mismatched_lifetime_syntaxes)]

struct Pair<'a, 'b>(&'a u8, &'b u8);
struct PairWithType<'a, 'b, T>(&'a T, &'b T);

macro_rules! repeated_hidden_paths_with_middle_ref {
    ($pair:ident, $middle:ty) => {
        ($pair, $middle, $pair)
        //~^ ERROR hiding or eliding a lifetime that's named elsewhere is confusing
    };
}

macro_rules! repeated_hidden_paths_in_repetition {
    ($($pair:ident),+ ; $middle:ty) => {
        ($($pair),+, $middle, $($pair),+)
        //~^ ERROR hiding or eliding a lifetime that's named elsewhere is confusing
    };
}

fn elided_missing(x: &u8) -> Pair {
    //~^ ERROR hiding a lifetime that's elided elsewhere is confusing
    Pair(x, x)
}

fn elided_full(x: &u8) -> PairWithType<u8> {
    //~^ ERROR hiding a lifetime that's elided elsewhere is confusing
    PairWithType(x, x)
}

fn named_missing<'a>(x: &'a u8) -> Pair {
    //~^ ERROR hiding a lifetime that's named elsewhere is confusing
    Pair(x, x)
}

fn named_full<'a>(x: &'a u8) -> PairWithType<u8> {
    //~^ ERROR hiding a lifetime that's named elsewhere is confusing
    PairWithType(x, x)
}

fn named_macro_non_contiguous<'a>(
    x: &'a u8,
) -> repeated_hidden_paths_with_middle_ref!(Pair, &'_ u8) {
    (Pair(x, x), x, Pair(x, x))
}

fn named_macro_repetition<'a>(
    x: &'a u8,
) -> repeated_hidden_paths_in_repetition!(Pair, Pair; &'_ u8) {
    (Pair(x, x), Pair(x, x), x, Pair(x, x), Pair(x, x))
}

fn main() {}
