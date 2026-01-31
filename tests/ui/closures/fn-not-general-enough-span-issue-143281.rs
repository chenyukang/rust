//! Test case for issue #143281
//! Error should point at the closure, not at `None`

fn test() -> Option<impl Fn(&String)> {
    if false {
        return None;
    }
    Some(|_| {})
    //~^ ERROR implementation of `Fn` is not general enough
    //~| ERROR implementation of `FnOnce` is not general enough
    //~| ERROR implementation of `Fn` is not general enough
    //~| ERROR implementation of `Fn` is not general enough
}

fn main() {}
