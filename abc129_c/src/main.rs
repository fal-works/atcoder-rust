use std::io::*;

const SP: u8 = b' ';
const LF: u8 = b'\n';

struct CharacterInput<'a> {
    locked_stdin: StdinLock<'a>,
    bytes_buffer: Vec<u8>,
}

impl<'a> CharacterInput<'a> {
    fn read(&mut self, delimiter: u8) -> &[u8] {
        self.bytes_buffer.clear();

        let mut len = self
            .locked_stdin
            .read_until(delimiter, &mut self.bytes_buffer)
            .unwrap();

        if len > 0 {
            len -= 1;
        }

        &self.bytes_buffer[0..len]
    }

    fn scan_u(&mut self, delimiter: u8) -> u32 {
        self.read(delimiter)
            .into_iter()
            .fold(0, |accumulator, byte| {
                (byte - b'0') as u32 + accumulator * 10
            })
    }
}

fn create_cin<'a>(input: StdinLock<'a>, buffer_capacity: usize) -> CharacterInput {
    CharacterInput {
        locked_stdin: input,
        bytes_buffer: Vec::with_capacity(buffer_capacity),
    }
}

fn println<T: std::string::ToString>(s: T) {
    println!("{}", s.to_string());
}

// ------------------------------------------------------------------------

fn get_pattern_count(stair_no: usize, table: &mut Vec<Option<u32>>) -> u32 {
    table[stair_no].unwrap_or_else(|| match stair_no {
        0 => 1,
        1 => {
            table[1] = Some(1);
            1
        }
        _ => {
            let pattern_count =
                (get_pattern_count(stair_no - 2, table) + get_pattern_count(stair_no - 1, table)) % 1000000007;
            table[stair_no] = Some(pattern_count);
            pattern_count
        }
    })
}

fn main() {
    let cin = stdin();
    let cin = &mut create_cin(cin.lock(), 32);

    let n = cin.scan_u(SP) as usize;
    let m = cin.scan_u(LF) as usize;

    let mut table = vec![None as Option<u32>; n + 1];
    table[0] = Some(1);

    for _ in 0..m {
        table[cin.scan_u(LF) as usize] = Some(0);
    }

    println(get_pattern_count(n, &mut table));
}
