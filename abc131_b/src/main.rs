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
    let (n, least) = parse_pair::<isize, isize>();

    let greatest = least + n - 1;

    let eaten_apple_taste = if greatest < 0 {
        greatest
    } else if least > 0 {
        least
    } else {
        0
    };

    let total_taste: isize = (least..greatest + 1).sum();

    println(total_taste - eaten_apple_taste);
}
