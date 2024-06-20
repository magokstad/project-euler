fn main() {
    let str = include_str!("../data/euler22.txt");
    println!("{}", calculate(str.trim()));
}

fn calculate(str: &str) -> usize {
    let mut vec = str.split(",")
        .map(|x| x.strip_prefix("\"").unwrap().strip_suffix("\"").unwrap())
        .collect::<Vec<_>>();
    vec.sort();

    vec.iter()
        .map(|x| x.as_bytes().iter().map(|x| (x + 1 - 'A' as u8) as usize).sum::<usize>())
        .enumerate()
        .map(|(i, x)| (i + 1) * x)
        .sum()
}


#[cfg(test)]
mod tests {
    use crate::calculate;

    #[test]
    fn test_calculate_aaabbb() {
        assert_eq!(calculate("\"AAA\",\"BBB\""), (1 + 1 + 1) * 1 + (2 + 2 + 2) * 2);
    }

    #[test]
    fn test_calculate_first_three() {
        assert_eq!(calculate(r#""MARY","PATRICIA","LINDA""#), (13 + 1 + 18 + 25) * 2 + (16 + 1 + 20 + 18 + 9 + 3 + 9 + 1) * 3 + (12 + 9 + 14 + 4 + 1) * 1);
    }
}