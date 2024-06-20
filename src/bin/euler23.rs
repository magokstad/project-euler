use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use crate::euler21::get_proper_divisors;

mod euler21;

fn main() {
    let mut abundant_queue = VecDeque::new();
    let mut abundant_set = HashSet::new();
    let mut sum = 0;

    for i in 1usize..=100_000 {
        if is_abundant(i) {
            abundant_queue.push_front(i);
            abundant_set.insert(i);
        }
    }

    'numbas:
    for i in 1usize..=28123 {
        let mut local = abundant_queue.clone();

        // check each abundant
        let mut abundant = local.pop_back();
        while abundant.is_some() && abundant.unwrap() <= i {
            // if it can be written as sum, go to next
            if abundant_set.contains(&(i - abundant.unwrap())) {
                continue 'numbas;
            }
            abundant = local.pop_back();
        }
        // else, add to sum
        sum += i
    }

    println!("The sum of all positive integers which cannot be written as the sum of two abundant numbers is {sum}");
}

fn is_abundant(n: usize) -> bool {
    get_proper_divisors(n).iter().sum::<usize>() > n
}