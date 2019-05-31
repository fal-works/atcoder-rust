use std::io::*;
use std::str::FromStr;

fn parse_line<T: FromStr>() -> T {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn parse_pair<T: FromStr, U: FromStr>() -> (T, U) {
    let line = parse_line::<String>();
    let mut iter = line.split_whitespace();
    (
        iter.next().unwrap().parse().ok().unwrap(),
        iter.next().unwrap().parse().ok().unwrap(),
    )
}

fn print_triple<T: ToString, U: ToString, V: ToString>(v: (T, U, V)) {
    println!(
        "{} {} {}",
        v.0.to_string(),
        v.1.to_string(),
        v.2.to_string()
    );
}

// ------------------------------------------------------------------------

struct ArithmeticProgression {
    value: usize,
    common_difference: usize,
}

impl ArithmeticProgression {
    fn new(initial_value: usize, common_difference: usize) -> ArithmeticProgression {
        assert!(common_difference > 0);
        ArithmeticProgression {
            value: initial_value,
            common_difference: common_difference,
        }
    }
}

impl Iterator for ArithmeticProgression {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.value;
        self.value += self.common_difference;
        Some(next)
    }
}

// ------------------------------------------------------------------------

type Bills = (usize, usize); // count, amount
type Pattern = Option<(isize, isize, isize)>; // count of 10000 Yen, 5000 Yen and 1000 Yen bills

fn find_valid_pattern<F>(unit_amount: usize, repetition: usize, validate: F) -> Pattern
where
    F: Fn(Bills) -> Pattern
{
    ArithmeticProgression::new(0, unit_amount)
        .take(repetition)
        .enumerate()
        .filter_map(validate)
        .next()
}

fn main() {
    let (n, y) = parse_pair::<usize, usize>();

    // If you know about 10k and 5k, you can derive the count of 1k and validate the pattern
    let validate_all = |bills_10k: Bills, bills_5k: Bills| -> Pattern {
        let (count_10k, amount_10k) = bills_10k;
        let (count_5k, amount_5k) = bills_5k;

        let count_1k = n - count_10k - count_5k;
        let amount_1k = 1000 * count_1k;

        if amount_10k + amount_5k + amount_1k == y {
            Some((count_10k as isize, count_5k as isize, count_1k as isize))
        } else {
            None
        }
    };

    // If you know about 10k, you know how to proceed the search
    let validate_10k = |bills_10k: Bills| -> Pattern {
        let count_10k = bills_10k.0;
        let remaining_count = n + 1 - count_10k;
        let validate_5k = |bills_5k| validate_all(bills_10k, bills_5k);

        find_valid_pattern(5000, remaining_count, validate_5k)
    };

    // Start search on 10k and you'll get the result
    let pattern = find_valid_pattern(10000, n + 1, validate_10k).unwrap_or((-1, -1, -1));

    print_triple(pattern);
}
