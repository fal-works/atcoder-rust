use std::io::*;
use std::str::FromStr;

fn parse_line<T: FromStr>() -> T {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

// ------------------------------------------------------------------------

fn main() {
    let a = parse_line::<u32>();

    println!("{}", (a - 2) * 180);
}
