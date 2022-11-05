fn foo() { //~ NOTE unclosed delimiter
  match Some(10) {
      Some(y) => { panic!(); }
      None => { panic!(); }
}

fn bar() {
    let mut i = 0;
    while (i < 1000) {}
}

fn main() {}
//~ ERROR this file contains an unclosed delimiter
