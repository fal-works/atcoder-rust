use std::io::*;

fn parse_bits() -> usize {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    usize::from_str_radix(s.trim(), 2).ok().unwrap()
}

// ------------------------------------------------------------------------

fn count_bits(bits: usize, len: usize) -> usize {
    let mut bits = bits;
    (0..len).fold(0, |acc, _| {
        let next = if bits & 1 == 1 { acc + 1 } else { acc };
        bits = bits >> 1;
        next
    })
}

// ------------------------------------------------------------------------

fn main() {
    println!("{}", count_bits(parse_bits(), 3));
}
