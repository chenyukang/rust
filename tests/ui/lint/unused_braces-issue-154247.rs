//@ edition: 2024
//@ check-pass

#![deny(unused_braces)]

use std::sync::Mutex;

fn consume(lock: &Mutex<Vec<u8>>, _value: usize) {
    let _guard = lock.lock().unwrap();
}

struct Lockable(Mutex<Vec<u8>>);

impl Lockable {
    fn update(&self, _value: usize) {
        let _guard = self.0.lock().unwrap();
    }

    fn run(&self) {
        // These blocks shorten the lifetime of the temporary `MutexGuard`.
        consume(&self.0, { self.0.lock().unwrap().len() });
        self.update({ self.0.lock().unwrap().len() });
    }
}

fn main() {
    Lockable(Mutex::new(vec![1])).run();
}
