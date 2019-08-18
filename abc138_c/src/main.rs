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
    let mut v = parse_line_vec::<f64>();

    v.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut iter = v.into_iter();
    let first = iter.next().unwrap();
    let result = iter.fold(first, |acc, cur| 0.5 * (acc + cur));

    println(result);
}
