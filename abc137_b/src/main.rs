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

fn print_from_iterator<T: ToString, I>(
    iterator: &mut I,
    separator: &str
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
    let (k, x) = parse_pair::<isize, isize>();

    let min = x - (k - 1);
    let max = x + (k - 1) + 1;
    let range = min..max;

    print_from_iterator(&mut range.into_iter(), " ");
}
