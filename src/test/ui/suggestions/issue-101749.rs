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
    let width = 3;
    let height = 4;
    let rect1 = Rectangle::new(width, height);
    println!("{}", rect1::area());
    //~^ ERROR failed to resolve: use of undeclared crate or module
}
