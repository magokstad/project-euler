fn main() {
    let mut longest = String::new();
    for i in 1..=1000 {
        let curr = num_to_word(i);
        println!("{}", curr);
        longest += curr.as_str();
    }
    println!("length: {}", longest.len());
}

fn num_to_word(num: usize) -> String {
    let mut word = String::new();
    let num_string = num.to_string();

    if num == 0 {
        return String::from("zero");
    }

    for (index, digit) in (0..num_string.len()).rev().zip(num_string.chars()) {
        match index {
            3 => {
                word += digit_to_word(digit);
                word += "thousand";
            }
            2 => {
                word += digit_to_word(digit);
                if digit_to_word(digit).len() != 0 {
                    word += "hundred"
                }
                if !(num_string.chars().rev().nth(1).unwrap() == '0'
                    && num_string.chars().rev().nth(0).unwrap() == '0')
                {
                    word += "and";
                }
            }
            1 => {
                let (hi, lo) = last_two(digit, num_string.chars().rev().nth(0).unwrap());
                word += hi;
                word += lo;
                break;
            }
            0 => word += digit_to_word(digit),
            _ => panic!("illegal index"),
        }
    }

    word
}

fn digit_to_word(digit: char) -> &'static str {
    match digit {
        '0' => "",
        '1' => "one",
        '2' => "two",
        '3' => "three",
        '4' => "four",
        '5' => "five",
        '6' => "six",
        '7' => "seven",
        '8' => "eight",
        '9' => "nine",
        _ => panic!("found non-digit token: {}", digit),
    }
}

fn last_two(dhi: char, dlo: char) -> (&'static str, &'static str) {
    if dhi == '1' {
        let teens = match dlo {
            '0' => "ten",
            '1' => "eleven",
            '2' => "twelve",
            '3' => "thirteen",
            '4' => "fourteen",
            '5' => "fifteen",
            '6' => "sixteen",
            '7' => "seventeen",
            '8' => "eighteen",
            '9' => "nineteen",
            _ => panic!("found non-digit token: {}", dhi),
        };
        return (teens, "");
    }

    let p1 = match dhi {
        '0' => "",
        '1' => panic!("teens should already have been handled"),
        '2' => "twenty",
        '3' => "thirty",
        '4' => "forty",
        '5' => "fifty",
        '6' => "sixty",
        '7' => "seventy",
        '8' => "eighty",
        '9' => "ninety",
        _ => panic!("found non-digit token: {}", dhi),
    };

    let p2 = digit_to_word(dlo);

    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_to_nine() {
        let (mut expected, mut actual);

        expected = "zero";
        actual = num_to_word(0);
        assert_eq!(expected, actual);

        expected = "one";
        actual = num_to_word(1);
        assert_eq!(expected, actual);

        expected = "two";
        actual = num_to_word(2);
        assert_eq!(expected, actual);

        expected = "three";
        actual = num_to_word(3);
        assert_eq!(expected, actual);

        expected = "four";
        actual = num_to_word(4);
        assert_eq!(expected, actual);

        expected = "five";
        actual = num_to_word(5);
        assert_eq!(expected, actual);

        expected = "six";
        actual = num_to_word(6);
        assert_eq!(expected, actual);

        expected = "seven";
        actual = num_to_word(7);
        assert_eq!(expected, actual);

        expected = "eight";
        actual = num_to_word(8);
        assert_eq!(expected, actual);

        expected = "nine";
        actual = num_to_word(9);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_teens() {
        let (mut expected, mut actual);

        expected = "ten";
        actual = num_to_word(10);
        assert_eq!(expected, actual);

        expected = "eleven";
        actual = num_to_word(11);
        assert_eq!(expected, actual);

        expected = "twelve";
        actual = num_to_word(12);
        assert_eq!(expected, actual);

        expected = "thirteen";
        actual = num_to_word(13);
        assert_eq!(expected, actual);

        expected = "fourteen";
        actual = num_to_word(14);
        assert_eq!(expected, actual);

        expected = "fifteen";
        actual = num_to_word(15);
        assert_eq!(expected, actual);

        expected = "sixteen";
        actual = num_to_word(16);
        assert_eq!(expected, actual);

        expected = "seventeen";
        actual = num_to_word(17);
        assert_eq!(expected, actual);

        expected = "eighteen";
        actual = num_to_word(18);
        assert_eq!(expected, actual);

        expected = "nineteen";
        actual = num_to_word(19);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tens() {

        let (mut expected, mut actual);

        expected = "ten";
        actual = num_to_word(10);
        assert_eq!(expected, actual);

        expected = "twenty";
        actual = num_to_word(20);
        assert_eq!(expected, actual);

        expected = "thirty";
        actual = num_to_word(30);
        assert_eq!(expected, actual);

        expected = "forty";
        actual = num_to_word(40);
        assert_eq!(expected, actual);

        expected = "fifty";
        actual = num_to_word(50);
        assert_eq!(expected, actual);

        expected = "sixty";
        actual = num_to_word(60);
        assert_eq!(expected, actual);

        expected = "seventy";
        actual = num_to_word(70);
        assert_eq!(expected, actual);

        expected = "eighty";
        actual = num_to_word(80);
        assert_eq!(expected, actual);

        expected = "ninety";
        actual = num_to_word(90);
        assert_eq!(expected, actual);

    }

    #[test]
    fn test_hundreds() {
        let (mut expected, mut actual);

        expected = "onehundred";
        actual = num_to_word(100);
        assert_eq!(expected, actual);

        expected = "twohundred";
        actual = num_to_word(200);
        assert_eq!(expected, actual);

        expected = "threehundred";
        actual = num_to_word(300);
        assert_eq!(expected, actual);

        expected = "fourhundred";
        actual = num_to_word(400);
        assert_eq!(expected, actual);

        expected = "fivehundred";
        actual = num_to_word(500);
        assert_eq!(expected, actual);

        expected = "sixhundred";
        actual = num_to_word(600);
        assert_eq!(expected, actual);

        expected = "sevenhundred";
        actual = num_to_word(700);
        assert_eq!(expected, actual);

        expected = "eighthundred";
        actual = num_to_word(800);
        assert_eq!(expected, actual);

        expected = "ninehundred";
        actual = num_to_word(900);
        assert_eq!(expected, actual);

    }

    #[test]
    fn test_thousands() {
        let (mut expected, mut actual);

        expected = "onethousand";
        actual = num_to_word(1000);
        assert_eq!(expected, actual);

        expected = "twothousand";
        actual = num_to_word(2000);
        assert_eq!(expected, actual);

        expected = "threethousand";
        actual = num_to_word(3000);
        assert_eq!(expected, actual);

        expected = "fourthousand";
        actual = num_to_word(4000);
        assert_eq!(expected, actual);

        expected = "fivethousand";
        actual = num_to_word(5000);
        assert_eq!(expected, actual);

        expected = "sixthousand";
        actual = num_to_word(6000);
        assert_eq!(expected, actual);

        expected = "seventhousand";
        actual = num_to_word(7000);
        assert_eq!(expected, actual);

        expected = "eightthousand";
        actual = num_to_word(8000);
        assert_eq!(expected, actual);

        expected = "ninethousand";
        actual = num_to_word(9000);
        assert_eq!(expected, actual);

    }

    #[test]
    fn mixture() {
        let (mut expected, mut actual);

        expected = "onethousandonehundredandeleven";
        actual = num_to_word(1111);
        assert_eq!(expected, actual);

        expected = "twothousandtwohundredandtwentytwo";
        actual = num_to_word(2222);
        assert_eq!(expected, actual);

        expected = "threethousandthreehundredandthirtythree";
        actual = num_to_word(3333);
        assert_eq!(expected, actual);
    }

    #[test]
    #[should_panic]
    fn test_digit_panic() {
        digit_to_word('a');
    }

    #[test]
    #[should_panic]
    fn test_teen_panic() {
        last_two('1', 'a');
    }

    #[test]
    #[should_panic]
    fn test_tens_panic() {
        last_two('a', '1');
    }

    #[test]
    #[should_panic]
    fn test_unsupported_length() {
        num_to_word(12345678);
    }
}