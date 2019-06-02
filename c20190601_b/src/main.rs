use std::io::*;
use std::str::FromStr;

fn parse_line<T: FromStr>() -> T {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

// ------------------------------------------------------------------------

fn main() {
    let s = parse_line::<String>();

    let passed_days = s.len();
    let wins = s.chars().into_iter().fold(0, |acc, c| if c =='o' { acc + 1 } else { acc });

    println!("{}", if (8 - wins) <= (15 - passed_days) { "YES" } else { "NO"} );
}
