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

fn average(a: u32, b: u32) -> u32 {
    (a + b) >> 1
}

fn main() {
    let (a, b) = parse_pair::<u32, u32>();

    if (a ^ b) & 1 == 1 {
        println("IMPOSSIBLE");
        return;
    }

    println(average(a, b));
}
