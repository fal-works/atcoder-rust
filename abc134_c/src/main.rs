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

struct CharacterOutput {
    string_buffer: String,
}

impl CharacterOutput {
    fn push_s(&mut self, s: &str) {
        self.string_buffer += s;
    }

    fn push<T: ToString>(&mut self, n: T) {
        self.push_s(&n.to_string());
    }

    fn add<T: ToString>(&mut self, n: T, separator: char) {
        self.push(n);
        self.string_buffer.push(separator);
    }

    fn flush(&mut self) {
        let out = stdout();
        let mut out = out.lock();
        out.write_all(self.string_buffer.as_bytes()).unwrap();
        self.string_buffer.clear();
    }
}

fn create_cout(capacity: usize) -> CharacterOutput {
    CharacterOutput {
        string_buffer: String::with_capacity(capacity),
    }
}

// ------------------------------------------------------------------------

fn main() {
    let cin = stdin();
    let cin = &mut create_cin(cin.lock(), 16);

    let mut max_a = 0;
    let mut second_max_a = 0;

    let n = cin.scan_u(LF) as usize;
    let a: Vec<u32> = (0..n)
        .map(|_| cin.scan_u(LF))
        .map(|a_i| {
            if a_i > max_a {
                max_a = a_i;
            } else if a_i > second_max_a {
                second_max_a = a_i;
            }

            a_i
        })
        .collect();

    let cout = &mut create_cout(7 * n);

    for a_i in a {
        if a_i < max_a { cout.add(max_a, '\n'); }
        else { cout.add(second_max_a, '\n') }
    }

    cout.flush();
}
