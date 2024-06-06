use std::collections::HashSet;
use fraction::BigUint;

fn main() {
    let mut set = HashSet::new();
    for a in 2..=100_u32 {
        for b in 2..=100_u32 {
            set.insert(BigUint::from(a).pow(b));
            set.insert(BigUint::from(b).pow(a));
        }
    }
    println!("There are {} unique powers",  set.len());
}