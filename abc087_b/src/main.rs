use std::io::*;

fn parse_line<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

// ------------------------------------------------------------------------

fn main() {
    let a = parse_line::<usize>();
    let b = parse_line::<usize>();
    let c = parse_line::<usize>();
    let x = parse_line::<usize>();

    let c_max_amount = c * 50;
    let mut valid_count = 0;

    let _: usize = (0..a + 1).fold(0, |a_amount, _| {
        let _: usize = (0..b + 1).fold(a_amount, |ab_amount, _| {
            if ab_amount <= x && x <= ab_amount + c_max_amount {
                valid_count += 1;
            }
            ab_amount + 100
        });
        a_amount + 500
    });

    println! {"{}", valid_count};
}
