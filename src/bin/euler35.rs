fn main() {
    let mut n_circulars = 0;

    'each_prime:
    for prime in primal::Primes::all().take(1_000_000) {
        let mut cand = prime.to_string();
        for _ in 0..prime.to_string().len() {
            if !primal::is_prime(cand.parse::<u64>().expect("cant parse")) {
                continue 'each_prime
            }
            cand = cand.rotate_left();
        }
        println!("{prime} is circular");
        n_circulars += 1;
    }
    println!("There are {} circular primes", n_circulars);
}

trait Rotate {
    fn rotate_right(self) -> Self;
    fn rotate_left(self) -> Self;
}
impl Rotate for String {
    fn rotate_right(self) -> Self {
        if self.is_empty() {
            return self;
        }
        self.chars().last().unwrap().to_string() + &self[..self.len() - 1]
    }

    fn rotate_left(self) -> Self {
        self.chars().skip(1)
            .chain(self.chars().take(1))
            .collect()
    }
}