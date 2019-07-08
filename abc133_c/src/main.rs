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
    let (l, r) = parse_pair::<usize, usize>();

    const DIVISOR: usize = 2019;

    let i_range = (l..r).take(DIVISOR);
    let mut min_mod = DIVISOR - 1;

    'outer: for i in i_range {
        let j_range = (i + 1..r + 1).take(DIVISOR);
        for j in j_range {
            let cur_mod = (i * j) % DIVISOR;
            if cur_mod < min_mod {
                min_mod = cur_mod;
                if min_mod == 0 {
                    break 'outer;
                }
            }
        }
    }

    println(min_mod);
}
