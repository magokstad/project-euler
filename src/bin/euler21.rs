use std::collections::HashSet;
use fraction::ToPrimitive;

fn main() {
    let mut sum = 0;
    for n in 1..10000 {
        if is_amicable(n) {
            sum += n;
        }
    }

    println!("The sum of all amicable numbers under 10000 is {sum}");
}

fn is_amicable(n: usize) -> bool {
    let potential_partner = get_proper_divisors(n).iter().sum::<usize>();
    let potentially_n = get_proper_divisors(potential_partner).iter().sum::<usize>();
    potentially_n == n && potential_partner != n
}

pub fn get_proper_divisors(n: usize) -> HashSet<usize> {
    let mut divisors = HashSet::new();
    let sqrt_num = n.to_f32().unwrap().sqrt().to_usize().unwrap();
    for div in 1..sqrt_num {
        if n % div == 0 {
            let other = n / div;
            if other != n {
                divisors.insert(other);
            }
            divisors.insert(div);
        }
    }

    divisors
}

#[cfg(test)]
mod tests {
    use crate::is_amicable;

    #[test]
    fn test_220_is_amicable() {
        assert!(is_amicable(220));
    }

    #[test]
    fn test_221_is_not_amicable() {
        assert!(!is_amicable(221));
    }
}