use std::sync::{Mutex};
use lazy_static::lazy_static;
use memoize::memoize;

lazy_static! {
    static ref ARRAY: Mutex<Vec<[usize;10]>> = Mutex::new(vec![]);
}

trait GlobalArr<T> {
    fn gpush(&self, n: T);
}

impl<T> GlobalArr<T> for Mutex<Vec<T>> {
    fn gpush(&self, n: T) {
        self.lock().expect("Failed to acquire lock").push(n);
    }
}

fn main() {}

#[memoize]
fn something(vec:  vec: Vec<usize>) {
    
}