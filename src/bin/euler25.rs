use fraction::BigUint;

pub struct Fibonacci {
    prev: BigUint,
    cur: BigUint,
}

impl Fibonacci {
    fn all() -> Self {
        Self {
            prev: BigUint::from(0u8),
            cur: BigUint::from(1u8),
        }
    }
}

impl Iterator for Fibonacci {
    type Item = BigUint;

    fn next(&mut self) -> Option<Self::Item> {
        let old_prev = self.prev.clone();
        let old_cur = self.cur.clone();

        self.prev = old_cur.clone();
        self.cur = old_prev + old_cur.clone();

        Some(old_cur)
    }
}

fn main() {
    for (i, f) in Fibonacci::all().enumerate() {
        if f.to_string().len() >= 1000 {
            println!("i: {i}");
            break;
        }
    }
}
