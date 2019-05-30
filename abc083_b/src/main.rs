use std::io::*;
use std::str::FromStr;

fn parse_line<T: FromStr>() -> T {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn parse_triple<T: FromStr, U: FromStr, V: FromStr>() -> (T, U, V) {
    let line = parse_line::<String>();
    let mut iter = line.split_whitespace();
    (
        iter.next().unwrap().parse().ok().unwrap(),
        iter.next().unwrap().parse().ok().unwrap(),
        iter.next().unwrap().parse().ok().unwrap(),
    )
}

// ------------------------------------------------------------------------

trait CheckRange<T: std::cmp::PartialOrd> {
    /** Naive alternative to contains(), which is not supported by v1.15.1 */
    fn includes(&self, n: T) -> bool;
}

impl <T: std::cmp::PartialOrd> CheckRange<T> for std::ops::Range<T> {
    fn includes(&self, n: T) -> bool {
        self.start <= n && n < self.end
    }
}

struct DigitIterator {
    value: usize,
    base: usize,
}

impl DigitIterator {
    fn new<T: Into<usize>>(value: T, base: usize) -> DigitIterator {
        assert!(base > 0);
        DigitIterator {
            value: value.into(),
            base: base,
        }
    }
}

impl Iterator for DigitIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value > 0 {
            let digit = self.value % self.base;
            self.value /= self.base;
            Some(digit)
        } else {
            None
        }
    }
}

trait Digits {
    fn digits(self, base: usize) -> DigitIterator;
}

impl <T: Into<usize>> Digits for T {
    fn digits(self, base: usize) -> DigitIterator {
        DigitIterator::new(self, base)
    }
}

// ------------------------------------------------------------------------

fn main() {
    let (n, a, b) = parse_triple::<usize, usize, usize>();
    let valid_range = a..b + 1;

    let sum: usize = (1..n + 1)
        .filter(|n| valid_range.includes(n.digits(10).sum()))
        .sum();

    println!("{}", sum);
}
