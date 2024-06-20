use std::collections::{HashSet, VecDeque};
use crate::euler21::get_proper_divisors;

mod euler21;

fn main() {
    let abundants = (1..=28123 * 2).filter(|x| is_abundant(*x)).collect::<Vec<usize>>();

    let mut sum_of_two = HashSet::new();
    let mut sum = 0;

    for abunl in &abundants {
        for abunr in &abundants {
            sum_of_two.insert(abunl + abunr);
        }
    }

    for i in 0..=28123 {
        if !sum_of_two.contains(&i) {
            sum += i;
        }
    }

    println!("The sum of all positive integers which cannot be written as the sum of two abundant numbers is {sum}");
}

fn is_abundant(n: usize) -> bool {
    get_proper_divisors(n).iter().sum::<usize>() > n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_abundant() {
        assert!(is_abundant(12));
        assert!(!is_abundant(15));
    }
}