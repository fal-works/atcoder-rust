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

fn read_bytes(length: usize, skip_next: bool) -> Vec<u8> {
    let mut bytes = vec![0; length];
    let stdin = stdin();
    let mut stdin = stdin.lock();
    stdin.read_exact(&mut bytes).unwrap();
    if skip_next {
        stdin.bytes().next();
    }
    bytes
}

fn println<T: std::string::ToString>(s: T) {
    println!("{}", s.to_string());
}

// ------------------------------------------------------------------------

#[derive(Copy, Clone)]
struct ConsecutiveBytes {
    unit_byte: u8,
    count: usize,
}

impl ConsecutiveBytes {
    fn new(unit_byte: u8) -> ConsecutiveBytes {
        ConsecutiveBytes {
            unit_byte: unit_byte,
            count: 0,
        }
    }

    fn count(&mut self, byte: u8) -> usize {
        if byte == self.unit_byte {
            self.count += 1;
        } else {
            self.count = 0;
        }

        self.count
    }
}

// ------------------------------------------------------------------------

fn main() {
    let header = parse_line_vec::<usize>();
    let n = header[0];
    let a = header[1] - 1; // zero-origin
    let b = header[2] - 1;
    let c = header[3] - 1;
    let d = header[4] - 1;
    let s = read_bytes(n, false);

    let mut rocks = ConsecutiveBytes::new(b'#');

    for index in a..std::cmp::max(c, d) {
        if rocks.count(s[index]) >= 2 {
            println("No");
            return;
        }
    }

    if c < d {
        // does not need to overtake
        println("Yes");
        return;
    }

    let mut spaces = ConsecutiveBytes::new(b'.');

    for index in b - 1..std::cmp::min(d + 2, n) {
        if spaces.count(s[index]) >= 3 {
            println("Yes");
            return;
        }
    }

    println("No");
}
