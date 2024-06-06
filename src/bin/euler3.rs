use std::collections::{BinaryHeap, HashSet};

fn main() {
    let num = 600851475143;
    let sqrt_num = (num as f64) .sqrt() as usize;

    let mut factors = HashSet::new();
    for prime in primal::Primes::all().take_while(|p| p < &sqrt_num) {
        if num % prime == 0 {
            let other = num / prime;
            factors.insert(other);
            factors.insert(prime);
        }
    }

    let mut sorted = BinaryHeap::from(factors.iter().collect::<Vec<&usize>>());
    let mut ans: usize = 0;

    'next_factor:
    for lnum in sorted {
        let sqrt_lnum = (*lnum as f64) .sqrt() as usize;
        for prime in primal::Primes::all().take_while(|p| p < &sqrt_lnum) {
            if lnum % prime == 0 {
                continue 'next_factor;
            }
        }
        if *lnum > ans {
            ans = *lnum;
        }
    }

    println!("Num: {num}");
    println!("sqrt(Num): {sqrt_num}");
    println!("Ans: {ans}")
}
