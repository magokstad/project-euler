use std::collections::HashSet;

use fraction::ToPrimitive;

fn main() {
    let x = thing();
    println!("{x} is right");
}

fn thing() -> usize {
    let mut num = 0;
    for x in 1.. {
        num += x;
        let sqrt_num = num.to_f64().unwrap().sqrt().to_usize().unwrap();

        let mut divisors = HashSet::new();
        for div in 1..sqrt_num {
            if num % div == 0 {
                let other = num / div;
                divisors.insert(other);
                divisors.insert(div);
            }
        }

        println!("{num} has {} divisors", divisors.len());
        if divisors.len() >= 500 {
            break;
        }
    }

    num
}
