use std::io::*;

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

fn println<T: ToString>(s: T) {
    println!("{}", s.to_string());
}

// ------------------------------------------------------------------------

fn main() {
    let s = read_bytes(4, false);

    for byte_pair in s.windows(2) {
        if byte_pair[0] == byte_pair[1] {
            println("Bad");
            return;
        }
    }

    println("Good");
}
