use std::io::*;
use std::str::FromStr;

fn parse_line<T: FromStr>() -> T {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn parse_lines<T: FromStr>(n: usize) -> Vec<T> {
    let stdin = stdin();
    let mut stdin = stdin.lock();

    let mut s = String::new();
    (0..n)
        .map(|_| {
            s.clear();
            stdin.read_line(&mut s).ok();
            s.trim().parse().ok().unwrap()
        })
        .collect()
}

// ------------------------------------------------------------------------

fn main() {
    let n = parse_line::<usize>();
    let d = parse_lines::<usize>(n);

    let mochimochi_diameter_set: std::collections::HashSet<usize> = d.into_iter().collect();

    println!("{}", mochimochi_diameter_set.len());
}
