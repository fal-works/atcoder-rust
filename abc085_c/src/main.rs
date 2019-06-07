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

trait PrintableIterator {
    fn println(self, length: usize, separator: &str);
}

impl<T> PrintableIterator for T
where
    T: IntoIterator,
    T::Item: ToString,
{
    fn println(self, length: usize, separator: &str) {
        let mut string_iterator = self.into_iter().map(|e| e.to_string());
        let stdout = stdout();
        let mut stdout = stdout.lock();

        for s in string_iterator.by_ref().take(length - 1) {
            stdout.write_all(s.as_bytes()).unwrap();
            stdout.write_all(separator.as_bytes()).unwrap();
        }
        stdout
            .write_all(string_iterator.next().unwrap().as_bytes())
            .unwrap();
        stdout.write_all("\n".as_bytes()).unwrap();
    }
}


// ------------------------------------------------------------------------

struct ArithmeticProgression {
    value: u32,
    common_difference: u32,
}

impl ArithmeticProgression {
    fn new(initial_value: u32, common_difference: u32) -> ArithmeticProgression {
        assert!(common_difference > 0);
        ArithmeticProgression {
            value: initial_value,
            common_difference: common_difference,
        }
    }
}

impl Iterator for ArithmeticProgression {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.value;
        self.value += self.common_difference;
        Some(next)
    }
}

// ------------------------------------------------------------------------

type Bills = (usize, u32); // count, amount
type Pattern = Vec<Bills>; // 10000 Yen, 5000 Yen and 1000 Yen bills

fn find_valid_pattern<F>(unit_amount: u32, repetition: usize, validate: F) -> Option<Pattern>
where
    F: Fn(Bills) -> Option<Pattern>,
{
    ArithmeticProgression::new(0, unit_amount)
        .take(repetition)
        .enumerate()
        .filter_map(validate)
        .next()
}

fn format_pattern(pattern: Option<Pattern>) -> Vec<isize> {
    match pattern {
        Some(bills_array) => bills_array
            .into_iter()
            .map(|bills| bills.0 as isize)
            .collect(),
        None => vec![-1; 3],
    }
}

fn main() {
    let (total_count, total_amount) = parse_pair::<usize, u32>(); // n, y

    // If you know about 10k and 5k, you can derive the count of 1k and validate the pattern
    let validate_all = |bills_10k: Bills, bills_5k: Bills| -> Option<Pattern> {
        let (count_10k, amount_10k) = bills_10k;
        let (count_5k, amount_5k) = bills_5k;

        let count_1k = total_count - count_10k - count_5k;
        let amount_1k = 1000 * count_1k as u32;

        if amount_10k + amount_5k + amount_1k == total_amount {
            Some(vec![bills_10k, bills_5k, (count_1k, amount_1k)])
        } else {
            None
        }
    };

    // If you know about 10k, you know how to proceed the search
    let validate_10k = |bills_10k: Bills| -> Option<Pattern> {
        let count_10k = bills_10k.0;
        let remaining_count = total_count - count_10k;
        let validate_5k = |bills_5k| validate_all(bills_10k, bills_5k);

        find_valid_pattern(5000, remaining_count + 1, validate_5k)
    };

    // Start search on 10k and you'll get the result
    format_pattern(find_valid_pattern(10000, total_count + 1, validate_10k))
        .into_iter()
        .println(3, " ");
}
