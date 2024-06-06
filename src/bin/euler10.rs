use std::collections::HashSet;

fn main() {
    let x: usize = primal::Primes::all().take_while(|p| p < &2_000).sum();
    println!("{x}");
    let z: usize = atkins_sieve(2_000).iter().sum();
    println!("{z}");
}

// DOES NOT WORK
fn eratosthenes_sieve(n: usize) -> HashSet<usize> {
    let rootn = (n as f64).sqrt() as usize;
    let mut set = HashSet::new();
    let mut check: Vec<bool> = (0..n).map(|_| true).collect();

    for i in 2..rootn {
        if *check.get(i).unwrap_or(&false) {
            for j in (0..).map(|x| i*i + x*i).take_while(|x| x < &n) {
                check.insert(j, false);
            }
        }
    }
    for (i,b) in check.iter().enumerate() {
        if i < 2 { continue; }
        if *b { set.insert(i); }
    }
    set
}

// DOES NOT WORK
fn atkins_sieve(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true;limit];
    let s = HashSet::from([1,7,11,13,17,19,23,29,31,37,41,43,47,49,53,59]);
    let mut out = vec![2,3,5];

    for x in &s {
        for n in (0..=limit/60).map(|w| w * 60 + x) {
            is_prime.insert(n, false);
        }
    }

    quadratic_one(limit, &mut is_prime);
    quadratic_two(limit, &mut is_prime);
    quadratic_three(limit, &mut is_prime);

    for x in &s {
        for n in (0..).map(|w| w * 60 + x).take_while(|n| n*n <= limit) {
            if n >= 7 && *is_prime.get(n).unwrap_or(&false) {
                for x2 in &s {
                    for c in (0..).map(|w| n*n * (w * 60 + x2)).take_while(|n| n <= &limit) {
                        is_prime.insert(c, false);
                    }
                }
            }
        }
    }

    for x in &s {
        for n in (0..).map(|w| w * 60 + x).skip_while(|n| n < &7).take_while(|n| n <= &limit) {
            if *is_prime.get(n).unwrap_or(&false) {
                out.push(n);
            }
        }
    }

    out

}

fn quadratic_one(limit: usize, is_prime: &mut Vec<bool>) {
    for x in (1..).take_while(|x| 4 * x*x <= limit) {
        for y in (1..).step_by(2).take_while(|y| 4 * x*x + y*y <= limit) {
            let n = 4 * x*x + y*y;
            match n % 60 {
                1 | 13 | 17 | 29 | 37 | 41 | 49 | 53 =>
                    is_prime.insert(n, !*is_prime.get(n).expect("shit1")),
                _ => ()
            }
        }
    }
}

fn quadratic_two(limit: usize, is_prime: &mut Vec<bool>, ) {
    for x in (1..).take_while(|x| 3 * x*x <= limit).step_by(2) {
        for y in (2..).step_by(2).take_while(|y| 4 * x*x + y*y <= limit) {
            let n = 3 * x*x + y*y;
            match n % 60 {
                7 | 19 | 31 | 43 =>
                    is_prime.insert(n, *is_prime.get(n).expect("shit2")),
                _ => ()
            }
        }
    }
}

fn quadratic_three(limit: usize, is_prime: &mut Vec<bool>, ) {
    for x in (2..).take_while(|x| 3 * x * x <= limit) {
        for y in (1..x).rev().step_by(2).take_while(|y| 4 * x*x + y*y <= limit) {
            let n = 3 * x*x - y*y;
            match n % 60 {
                11 | 23 | 47 | 59 =>
                    is_prime.insert(n, *is_prime.get(n).expect("shit3")),
                _ => ()
            }
        }
    }
}
