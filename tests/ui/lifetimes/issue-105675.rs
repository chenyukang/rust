fn thing(x: impl FnOnce(&u32, &u32, u32)) {}

fn main() {
    let f = | _ , y: &u32 , z | ();
    //~^ ERROR implementation of `FnOnce` is not general enough
    //~| ERROR implementation of `FnOnce` is not general enough
    thing(f);
    let f = | x, y: _  , z: u32 | ();
    //~^ ERROR implementation of `FnOnce` is not general enough
    //~| ERROR implementation of `FnOnce` is not general enough
    //~| ERROR implementation of `FnOnce` is not general enough
    //~| ERROR implementation of `FnOnce` is not general enough
    thing(f);
}
