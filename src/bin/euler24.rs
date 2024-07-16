use lazy_static::lazy_static;
use memoize::memoize;
use std::{collections::LinkedList, sync::Mutex};

lazy_static! {
    static ref ARRAY: Mutex<Vec<[usize; 10]>> = Mutex::new(vec![]);
}

trait GlobalArr<T> {
    fn gpush(&self, n: T);
}

impl<T> GlobalArr<T> for Mutex<Vec<T>> {
    fn gpush(&self, n: T) {
        self.lock().expect("Failed to acquire lock").push(n);
    }
}

fn main() {
    let ans = something(vec![], vec![0, 1, 2]);
    println!("{:?}", ans[0]);
    println!("{:?}", ans[1_000_000]);
}

const NUMBERS: usize = 3;

fn something(cur: Vec<u8>, left: Vec<u8>) -> Vec<[u8; NUMBERS]> {
    let mut vec: Vec<[u8; NUMBERS]> = Vec::new();

    if cur.len() == NUMBERS {
        let mut thing: [u8; NUMBERS] = [0; NUMBERS];
        for i in 0..NUMBERS {
            thing[i] = cur[i];
            vec.push(thing);
        }

        println!("done");
        return vec;
    }

    for i in 0..NUMBERS {
        let mut cur2 = cur
            .iter()
            .chain(left.iter().skip(i).take(1))
            .map(|x| x.to_owned())
            .collect();
        let mut left2 = left
            .iter()
            .take(i)
            .chain(left.iter().skip(i + 1))
            .map(|x| x.to_owned())
            .collect();
        println!("cur: {:?}", cur2);
        println!("left: {:?}", left2);
        vec.append(&mut something(cur2, left2));
    }

    vec
}
