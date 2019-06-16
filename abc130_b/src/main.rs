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
    let (n, x) = parse_pair::<usize, usize>();
    let l = parse_line_vec::<usize>();

    let mut acc = 0;

    for i in 0..n {
        let next = acc + l[i];
        if next > x {
            println(i + 1);
            return;
        }
        acc = next;
    }

    println(n + 1);
}
