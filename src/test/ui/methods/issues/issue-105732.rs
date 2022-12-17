#![feature(auto_traits)]

auto trait Foo {
    fn g(&self); //~ ERROR auto traits cannot have associated items
}

trait Bar {
    fn f(&self) {
        self.g(); //~ ERROR no method named `g` found for reference `&Self` in the current scope
    }
}

fn main() {}
