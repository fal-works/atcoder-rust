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

fn println<T: std::string::ToString>(s: T) {
    println!("{}", s.to_string());
}

// ------------------------------------------------------------------------

type Point = Vec<i32>;

fn squared_magnitude(p1: &Point, p2: &Point) -> u32 {
    p1.into_iter()
        .zip(p2)
        .map(|(x1, x2)| (x1 - x2).pow(2) as u32)
        .sum::<u32>()
}

fn distance(p1: &Point, p2: &Point) -> f32 {
    (squared_magnitude(p1, p2) as f32).sqrt()
}

fn main() {
    let (n, _d) = parse_pair::<usize, usize>();

    let x: Vec<Point> = (0..n).map(|_| parse_line_vec()).collect();

    let valid_count = (0..n - 1)
        .map(|i| {
            (i + 1..n)
                .map(|j| distance(&x[i], &x[j]))
                .filter(|dist| dist.fract() == 0.0)
                .count()
        })
        .sum::<usize>();

    println(valid_count);
}
