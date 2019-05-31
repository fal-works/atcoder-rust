use std::io::*;
use std::str::FromStr;

fn parse_line<T: FromStr>() -> T {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

// ------------------------------------------------------------------------

fn main() {
    let s: &str = &parse_line::<String>();

    let words = ["dream", "dreamer", "erase", "eraser"];
    let try_drop_tail = |s: &str| -> Option<usize> {
        // find_map() not supported by v1.15.1
        words.into_iter().filter_map(|word| {
            if s.ends_with(word) {
                Some(s.len() - word.len())
            } else {
                None
            }
        }).next()
    };

    let mut sub_str = s;

    while sub_str.len() > 0 {
        match try_drop_tail(sub_str) {
            Some(val) => sub_str = &s[0..val],
            None => {
                println!("NO");
                return;
            }
        };
    }

    println!("YES");
}
