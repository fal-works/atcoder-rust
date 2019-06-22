use std::io::*;
use std::str::FromStr;

fn parse_line<T: FromStr>() -> T {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn parse_quadraple<T: FromStr, U: FromStr, V: FromStr, W: FromStr>() -> (T, U, V, W) {
    let line = parse_line::<String>();
    let mut iter = line.split_whitespace();
    (
        iter.next().unwrap().parse().ok().unwrap(),
        iter.next().unwrap().parse().ok().unwrap(),
        iter.next().unwrap().parse().ok().unwrap(),
        iter.next().unwrap().parse().ok().unwrap(),
    )
}

fn println<T: ToString>(s: T) {
    println!("{}", s.to_string());
}

// ------------------------------------------------------------------------

// greatest common divisor
fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}

// least common multiple
fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

// ------------------------------------------------------------------------

fn count_dividable(start: u64, last: u64, divisor: u64) -> u64 {
    let start_mod = start % divisor;

    if start == last {
        return if start_mod == 0 { 1 } else { 0 };
    }

    let first_dividable = if start_mod == 0 {
        start
    } else {
        start + divisor - start_mod
    };

    if first_dividable <= last {
        1 + (last - first_dividable) / divisor
    } else {
        0
    }
}

fn main() {
    let (a, b, c, d) = parse_quadraple::<u64, u64, u64, u64>();

    let dividable_count = if c != d {
        count_dividable(a, b, c) + count_dividable(a, b, d) - count_dividable(a, b, lcm(c, d))
    } else {
        count_dividable(a, b, c)
    };

    let total_count = b + 1 - a;

    println(total_count - dividable_count);
}
