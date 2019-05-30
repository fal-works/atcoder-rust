use std::io::*;
use std::str::FromStr;

fn parse_line<T: FromStr>() -> T {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn parse_line_vec<T: FromStr>() -> Vec<T> {
    let line = parse_line::<String>();
    line.split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

// ------------------------------------------------------------------------

trait Integer {
    fn is_even(self) -> bool;
}

impl <T: Into<usize>> Integer for T {
    fn is_even(self) -> bool {
        self.into() & 1 == 0
    }
}

// ------------------------------------------------------------------------

fn main() {
    let n = parse_line::<usize>();
    let mut a = parse_line_vec::<usize>();

    // sort by descending order
    a.sort_by(|a, b| b.partial_cmp(a).unwrap()); // sort_unstable_by() is not supported by v1.15.1

    let mut alice_points: usize = 0;
    let mut bob_points: usize = 0;

    for index in 0..n {
        if index.is_even() {
            alice_points += a[index];
        } else {
            bob_points += a[index];
        }
    }

    println!("{}", alice_points - bob_points);
}
