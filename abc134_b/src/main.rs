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

fn println<T: ToString>(s: T) {
    println!("{}", s.to_string());
}

// ------------------------------------------------------------------------

fn divide_ceil(a: usize, b: usize) -> usize {
    (a + b - 1) / b
}

fn main() {
    let (n, d) = parse_pair::<usize, usize>();

    let range_length = 2 * d + 1;

    println(divide_ceil(n, range_length));
}
