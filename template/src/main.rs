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

        let len = self
            .locked_stdin
            .read_until(delimiter, &mut self.bytes_buffer)
            .unwrap();

        if len > 0 {
            let len = len - 1;
            if self.bytes_buffer[len] == delimiter {
                self.bytes_buffer.truncate(len);
            }
        }

        &self.bytes_buffer
    }

    fn scan_u(&mut self, delimiter: u8) -> u32 {
        self.read(delimiter)
            .into_iter()
            .fold(0, |accumulator, byte| {
                (byte - b'0') as u32 + accumulator * 10
            })
    }

    fn scan_u_vec(&mut self, n: usize, separator: u8, delimiter: u8) -> Vec<u32> {
        let mut vector = Vec::with_capacity(n as usize);
        for _i in 0..n - 1 {
            vector.push(self.scan_u(separator));
        }
        vector.push(self.scan_u(delimiter));
        vector
    }

    fn scan_u_pair(&mut self, separator: u8, delimiter: u8) -> (u32, u32) {
        (self.scan_u(separator), self.scan_u(delimiter))
    }

    fn scan_u_triple(&mut self, separator: u8, delimiter: u8) -> (u32, u32, u32) {
        (
            self.scan_u(separator),
            self.scan_u(separator),
            self.scan_u(delimiter),
        )
    }

    fn scan_s(&mut self, delimiter: u8) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.read(delimiter)) }
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

    fn add_s(&mut self, s: &str, separator: &str) {
        self.push_s(s);
        self.push_s(separator);
    }

    fn push<T: ToString>(&mut self, n: T) {
        self.push_s(&n.to_string());
    }

    fn add<T: ToString>(&mut self, n: T, separator: &str) {
        self.push(n);
        self.push_s(separator);
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

// easy
fn println(s: &str) {
    let stdout = stdout();
    let mut locked_stdout = stdout.lock();
    locked_stdout.write_all(s.as_bytes()).unwrap();
    locked_stdout.write_all(b"\n").unwrap();
    locked_stdout.flush().unwrap();
}

// ------------------------------------------------------------------------

fn process(cin: &mut CharacterInput, cout: &mut CharacterOutput) {
    
}

// ------------------------------------------------------------------------

fn main() {
    let cin = stdin();
    let cin = &mut create_cin(cin.lock(), 64);
    let cout = &mut create_cout(0);

    process(cin, cout);

    cout.flush();
}
