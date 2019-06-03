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

fn println<T: std::string::ToString>(s: T) {
    println!("{}", s.to_string());
}

// ------------------------------------------------------------------------

fn main() {
    let input = parse_line_vec::<usize>();
    let n = input[0];
    let a = input[1] - 1; // zero-origin
    let b = input[2] - 1;
    let c = input[3] - 1;
    let d = input[4] - 1;

    let mut s = Vec::with_capacity(n + 1);
    let stdin = stdin();
    stdin.lock().read_until(b'\n', &mut s).unwrap();

    const SPACE: u8 = b'.';
    const ROCK: u8 = b'#';

    let need_overtake = c > d;
    let mut can_overtake = false;

    let mut before_last = s[a];
    let mut before = s[a + 1];

    let last_overtake_index = std::cmp::min(c, d) + 1;

    for index in a + 2..std::cmp::max(c, d) {
        let current = s[index];

        if current == ROCK && before == ROCK {
            println("No");
            return;
        }

        if index > b && index <= last_overtake_index && current == SPACE && before == SPACE && before_last == SPACE {
            can_overtake = true;
        }

        before_last = before;
        before = current;
    }

    let ok = !need_overtake || can_overtake;

    println(if ok { "Yes" } else { "No" });
}
