use std::io::*;

fn parse_line<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn parse_line_vec<T: std::str::FromStr>() -> Vec<T> {
    let line = parse_line::<String>();
    line.split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

// ------------------------------------------------------------------------

fn is_even(n: usize) -> bool {
    (n & 1) == 0
}

fn map_parity<T>(n: usize, even_value: T, odd_value: T) -> T {
    if is_even(n) {
        even_value
    } else {
        odd_value
    }
}

fn product(numbers: Vec<usize>) -> usize {
    numbers
        .iter()
        .fold(1, |accumulator, current_value| accumulator * current_value)
}

// ------------------------------------------------------------------------

fn main() {
    println!("{}", map_parity(product(parse_line_vec()), "Even", "Odd"));
}
