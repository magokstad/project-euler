use fraction::BigUint;

fn main() {
    let mut big = BigUint::from(1u8);
    for i in 1..=100 {
        big *= BigUint::from(i as u8);
    }

    let ans: usize = big.to_string().chars().map(|x| x.to_digit(10).unwrap() as usize).sum();
    println!("{}", ans);
}