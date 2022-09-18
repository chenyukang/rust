// run-rustfix
struct Rectangle {
    width: i32,
    height: i32,
}
impl Rectangle {
    fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }
    fn area(&self) -> i32 {
        self.height * self.width
    }
}

fn main() {
    let rect = Rectangle::new(3, 4);
    //~^ WARNING ident `rect` is defined at here
    let _ = rect::area();
    //~^ ERROR failed to resolve: use of undeclared crate or module `rect`
    //~| ERROR `rect` is not a crate or module
}
