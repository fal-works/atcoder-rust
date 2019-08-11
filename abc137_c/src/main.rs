use std::io::*;

// const SP: u8 = b' ';
const LF: u8 = b'\n';

struct CharacterInput<'a> {
    locked_stdin: StdinLock<'a>,
    bytes_buffer: Vec<u8>,
}

impl<'a> CharacterInput<'a> {
    fn read(&mut self, delimiter: u8) -> &[u8] {
        self.bytes_buffer.clear();

        let len = self
            .locked_stdin
            .read_until(delimiter, &mut self.bytes_buffer)
            .unwrap();

        let end_index = match self.bytes_buffer.last() {
            Some(byte) => {
                if *byte == delimiter {
                    len - 1
                } else {
                    len
                }
            }
            None => len,
        };

        &self.bytes_buffer[0..end_index]
    }

    fn scan_u(&mut self, delimiter: u8) -> u32 {
        self.read(delimiter)
            .into_iter()
            .fold(0, |accumulator, byte| {
                (byte - b'0') as u32 + accumulator * 10
            })
    }

    fn scan_s(&mut self, delimiter: u8) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.read(delimiter)) }
    }

    fn scan_chars(&mut self, delimiter: u8) -> Vec<char> {
        self.scan_s(delimiter).chars().collect()
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

fn main() {
    let cin = stdin();
    let cin = &mut create_cin(cin.lock(), 16);

    let n = cin.scan_u(LF) as usize;

    let mut s: Vec<Vec<char>> = (0..n)
        .map(|_| {
            let mut chars = cin.scan_chars(LF);
            chars.sort();
            chars
        })
        .collect();
    s.sort();

    let mut total_count: u64 = 0;
    let mut current_count: u64 = 0;
    for slice in s.windows(2) {
        if slice[0] == slice[1] {
            current_count += 1;
            total_count += current_count;
        } else {
            current_count = 0;
        }
    }

    println(total_count);
}
