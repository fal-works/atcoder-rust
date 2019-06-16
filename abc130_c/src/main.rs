use std::io::*;
use std::str::FromStr;

fn parse_line<T: FromStr>() -> T {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
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

// ------------------------------------------------------------------------

fn main() {
    let (w, h, x, y) = parse_quadraple::<f64, f64, f64, f64>();

    let (center_x, center_y) = (w / 2.0, h / 2.0);

    println!("{} {}", w * h / 2.0, if x == center_x && y == center_y { 1 } else { 0 });
}
