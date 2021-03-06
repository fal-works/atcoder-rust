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

fn parse_triple<T: FromStr, U: FromStr, V: FromStr>() -> (T, U, V) {
    let line = parse_line::<String>();
    let mut iter = line.split_whitespace();
    (
        iter.next().unwrap().parse().ok().unwrap(),
        iter.next().unwrap().parse().ok().unwrap(),
        iter.next().unwrap().parse().ok().unwrap(),
    )
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

fn parse_line_vec<T: FromStr>() -> Vec<T> {
    let line = parse_line::<String>();
    line.split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

fn parse_lines<T: FromStr>(n: usize) -> Vec<T> {
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

fn parse_bits() -> usize {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    usize::from_str_radix(s.trim(), 2).ok().unwrap()
}

fn read_bytes(length: usize, skip_next: bool) -> Vec<u8> {
    let mut bytes = vec![0; length];
    let stdin = stdin();
    let mut stdin = stdin.lock();
    stdin.read_exact(&mut bytes).unwrap();
    if skip_next {
        stdin.bytes().next();
    }
    bytes
}

fn println<T: ToString>(s: T) {
    println!("{}", s.to_string());
}

fn print_from_iterator<T: ToString, I>(
    iterator: &mut I,
    length: usize,
    separator: &str,
    buffer_capacity: usize
) where
    I: Iterator<Item = T>,
{
    println!(
        "{}",
        iterator
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(separator),
    );
}

// ------------------------------------------------------------------------

fn main() {
    let n = parse_line::<usize>();

    

    println(n);
}
