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

fn println<T: ToString>(s: T) {
    println!("{}", s.to_string());
}

// ------------------------------------------------------------------------

fn main() {
    let _n = parse_line::<usize>();
    let p = parse_line_vec::<usize>();

    let valid = p
        .into_iter()
        .enumerate()
        .filter(|&(i, n)| i + 1 != n)
        .nth(2)
        .is_some();

    println(if valid { "NO" } else { "YES" });
}
