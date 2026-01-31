fn thing(x: impl FnOnce(&u32)) {}

fn main() {
    let f = |_| ();
    //~^ ERROR implementation of `FnOnce` is not general enough
    //~| ERROR implementation of `FnOnce` is not general enough
    thing(f);
}
