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

fn println<T: ToString>(s: T) {
    println!("{}", s.to_string());
}

// ------------------------------------------------------------------------

fn main() {
    let (m, d) = parse_pair::<usize, usize>();

    let min_d = 22;
    let max_d = d + 1;
    let min_d_1 = 2;
    let min_m = 4;
    let max_m = m + 1;

    let count = (min_d..max_d).fold(0, |acc, cd| {
        let d_1 = cd % 10;
        if d_1 < min_d_1 {
            return acc;
        }

        let d_10 = cd / 10;
        let d_product = d_1 * d_10;

        acc + (min_m..max_m).filter(|&m| m == d_product).count()
    });

    println(count);
}
