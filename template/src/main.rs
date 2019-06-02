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

    fn add_s(&mut self, s: &str, separator: char) {
        self.push_s(s);
        self.string_buffer.push(separator);
    }

    fn push<T: ToString>(&mut self, n: T) {
        self.push_s(&n.to_string());
    }

    fn add<T: ToString>(&mut self, n: T, separator: char) {
        self.push(n);
        self.string_buffer.push(separator);
    }

    fn add_from_iterator<T: ToString, I>(
        &mut self,
        iterator: &mut I,
        length: usize,
        separator: char,
        delimiter: char,
    ) where
        I: Iterator<Item = T>,
    {
        for element in iterator.by_ref().take(length - 1) {
            self.add(element, separator);
        }
        self.add(iterator.next().unwrap(), delimiter);
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
fn println<T: std::string::ToString>(s: T) {
    println!("{}", s.to_string());
}

fn print_from_iterator<T: ToString, I>(
    iterator: &mut I,
    length: usize,
    separator: &str,
    buffer_capacity: usize
) where
    I: Iterator<Item = T>,
{
    println!(
        "{}",
        iterator
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(separator),
    );
}

// ------------------------------------------------------------------------

fn main() {
    let cin = stdin();
    let cin = &mut create_cin(cin.lock(), 64);
    let cout = &mut create_cout(0);

    let n = cin.scan_u(LF) as usize;
    let a = cin.scan_u_vec(n, SP, LF);



    cout.flush();
}
