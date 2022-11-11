// ignore-tidy-trailing-newlines
//
// error-pattern: this file contains an unclosed delimiter
#![feature(let_chains)]
trait Demo {}

impl dyn Demo {
    pub fn report(&self,
                a: u32,
                b: u32,
                c: u32) -> u32 {
        return a + b + c;
    }

    fn check(&self, val: Option<u32>, num: Option<u32>) {
        if let Some(b) = val
        && let Some(c) = num {
        && b == c {
        }
    }
}

fn main() { } //~ ERROR this file contains an unclosed delimiter