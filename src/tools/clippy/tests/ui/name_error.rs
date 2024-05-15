use core::cell::Cell;

thread_local! {
    static fooFOO: Cell<usize> = unreachable!();
}

fn main() {
    fooFOO.set(9);
    println!("{}", fooFOO.get());
}
