use std::io::*;

fn parse_line<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn parse_pair<T: std::str::FromStr, U: std::str::FromStr>() -> (T, U) {
    let line = parse_line::<String>();
    let mut iter = line.split_whitespace();
    (
        iter.next().unwrap().parse().ok().unwrap(),
        iter.next().unwrap().parse().ok().unwrap(),
    )
}

fn parse_triple<T: std::str::FromStr, U: std::str::FromStr, V: std::str::FromStr>() -> (T, U, V) {
    let line = parse_line::<String>();
    let mut iter = line.split_whitespace();
    (
        iter.next().unwrap().parse().ok().unwrap(),
        iter.next().unwrap().parse().ok().unwrap(),
        iter.next().unwrap().parse().ok().unwrap(),
    )
}

fn parse_line_vec<T: std::str::FromStr>() -> Vec<T> {
    let line = parse_line::<String>();
    line.split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

fn parse_lines<T: std::str::FromStr>(n: usize) -> Vec<T> {
    let stdin = stdin();
    let mut stdin = stdin.lock();

    (0..n)
        .map(|_| {
            let mut s = String::new();
            stdin.read_line(&mut s).ok();
            s
        })
        .map(|s| s.trim().parse().ok().unwrap())
        .collect()
}

fn print_lines<T: std::string::ToString>(lines: Vec<T>) {
    println!(
        "{}",
        lines
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );
}

// ------------------------------------------------------------------------

fn main() {}
