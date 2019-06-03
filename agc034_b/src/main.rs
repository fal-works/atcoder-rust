use std::io::*;

fn println<T: std::fmt::Display>(s: T) {
    println!("{}", s);
}

// ------------------------------------------------------------------------

/** Replaces B, C to D */
fn replace_bytes(bytes: &mut Vec<u8>) {
    const B: u8 = b'B';
    const C: u8 = b'C';
    const D: u8 = b'D';

    let mut read_index = 0;
    let mut write_index = 0;

    let len = bytes.len();
    let replace_end_index = len - 1;

    while read_index < len {
        if read_index < replace_end_index && bytes[read_index] == B && bytes[read_index + 1] == C {
            bytes[write_index] = D;
            read_index += 2;
        } else {
            bytes[write_index] = bytes[read_index];
            read_index += 1;
        }
        write_index += 1;
    }
    bytes.resize(write_index, 0);
    bytes.shrink_to_fit();
}

fn count_contiguous_a_and_d(
    bytes: &Vec<u8>,
    start_index: usize,
    end_index: usize,
) -> (usize, Vec<usize>) {
    const A: u8 = b'A';
    const D: u8 = b'D';
    let mut d_position_list: Vec<usize> = Vec::new();

    if bytes[start_index] != A {
        return (0, d_position_list);
    }

    let mut len = 1;

    for index in start_index + 1..end_index {
        if bytes[index] == A {
            len += 1;
            continue;
        }

        if bytes[index] == D {
            d_position_list.push(len);
            len += 1;
            continue;
        }

        break;
    }

    return (len, d_position_list);
}

fn main() {
    let stdin = stdin();
    let mut s: Vec<u8> = stdin.lock().bytes().map(|b| b.unwrap()).collect();

    if s.len() < 3 {
        println(0);
        return;
    }

    replace_bytes(&mut s);  // BC -> D
    let len = s.len();

    const A: u8 = b'A';

    let mut operation_count = 0;
    let mut index = 0;

    while index < len {
        if s[index] != A {
            index += 1;
            continue;
        }

        let (a_and_d_len, d_position_list) = count_contiguous_a_and_d(&s, index, len);

        index += a_and_d_len;

        operation_count += d_position_list
            .into_iter()
            .enumerate()
            .fold(0, |acc, cur| acc + cur.1 - cur.0);
    }

    println(operation_count);
}
