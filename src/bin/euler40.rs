const DIGITS: [usize; 7] = [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000];

fn main() {
    let champs = construct_champernowne(1_000_000);
    let chars = champs.chars().collect::<Vec<char>>();
    let mut prod: usize = 1;
    for d in DIGITS {
        prod *= chars[d].to_digit(10).unwrap() as usize;
    }

    println!("{prod}");
}

fn construct_champernowne(min_index: usize) -> String {
    let mut champs = String::new();
    let mut num = 0;
    while champs.len() < min_index + 1 {
        champs += num.to_string().as_str();
        num += 1;
    }

    champs
}

#[cfg(test)]
mod tests {
    use crate::construct_champernowne;

    #[test]
    fn test_digits() {
        let champs = construct_champernowne(1_000_000);
        let chars = champs.chars().collect::<Vec<char>>();
        let get = |x: char| x.to_digit(10).unwrap() as usize;
        assert_eq!(get(chars[1]), 1);
        assert_eq!(get(chars[8]), 8);
        assert_eq!(get(chars[10]), 1);
        assert_eq!(get(chars[11]), 0);
        assert_eq!(get(chars[12]), 1);
        assert_eq!(get(chars[13]), 1);
    }
}