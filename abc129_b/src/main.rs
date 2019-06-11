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
    let n = parse_line::<usize>();
    let w = parse_line_vec::<i32>();

    let total_weight: i32 = w.iter().sum();

    let min_abs_difference = (1..n)
        .map(|i| {
            let former_part_weight: i32 = w[0..i].into_iter().sum();
            let latter_part_weight: i32 = total_weight - former_part_weight;
            (former_part_weight - latter_part_weight).abs()
        })
        .min()
        .unwrap();

    println(min_abs_difference);
}
