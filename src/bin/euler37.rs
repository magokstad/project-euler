fn main() {
    let mut sum = 0;
    let mut found = 0;

    'primes:
    for prime in primal::Primes::all() {
        // Trunc right to left
        let mut str = prime.to_string();
        while str.len() > 0 {
            let num = str.parse().unwrap();
            if !primal::is_prime(num) {
                continue 'primes;
            }
            str = str.chars().skip(1).collect();
        }

        // Trunc left to right
        let mut str = prime.to_string();
        while str.len() > 0 {
            let num = str.parse().unwrap();
            if !primal::is_prime(num) {
                continue 'primes;
            }
            str.pop();
        }

        if prime > 7 {
            println!("{} works", prime);
            found += 1;
            sum += prime;
        } else {
            println!("{} works, but doesn't count", prime);
        }

        if found == 11 {
            println!("The sum of the truncatable primes is {sum}");
            return;
        }
    }
}