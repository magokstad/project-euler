use fraction::ToPrimitive;

fn main() {
    let mut prod = 0;
    let mut max = 0;
    for a in -999..1000 {
        for b in -1000..=1000 {
            let x = primes_in_quadratic(a,b);
            if x > max {
                prod = a * b;
            }
        }
    }
    println!("{prod}");
}

fn primes_in_quadratic(a: i64, b: i64) -> u64 {
    let mut n = 0;
    let mut p = n*n + a*n + b;
    if p < 0 { return 0 }
    while primal::is_prime(p.to_u64().expect(&*format!("{p}"))) {
        n += 1;
        p = n*n + a*n + b;
        if p < 0 { return 0 }
    }
    n as u64
}