//@ check-pass

#![warn(unused)]

fn main() {
    let mut a = 10.;
    //~^ WARNING variable `a` is assigned to, but never used
    let b = 13.;
    let c = 11.;

    a += b;
    a -= c;
    //~^ WARNING value assigned to `a` is never read
}
