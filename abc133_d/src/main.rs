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

    fn scan_u_vec(&mut self, n: usize, separator: u8, delimiter: u8) -> Vec<u32> {
        let mut vector = Vec::with_capacity(n as usize);
        for _i in 0..n - 1 {
            vector.push(self.scan_u(separator));
        }
        vector.push(self.scan_u(delimiter));
        vector
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

// ------------------------------------------------------------------------

trait StepSum: Iterator {
    fn step_sum(&mut self, step: usize) -> u32;
}

impl<I: Iterator> StepSum for I where I: Iterator<Item = u32> {
    fn step_sum(&mut self, step: usize) -> u32 {
        let mut acc;

        match self.next() {
            Some(value) => acc = value,
            None => return 0
        }

        loop {
            match self.nth(step - 1) {
                Some(value) => acc += value,
                None => break
            }
        }

        acc
    }    
}

fn main() {
    let cin = stdin();
    let cin = &mut create_cin(cin.lock(), 32);

    let n = cin.scan_u(LF) as usize;
    let a = cin.scan_u_vec(n, SP, LF);

    let total_rainfall = a.iter().sum::<u32>();
    let first_x = total_rainfall - 2 * a.iter().skip(1).map(|&v| v).step_sum(2);
    let mut rest_x_iterator = a.iter().take(n - 1).scan(first_x, |x, a| {
        *x = 2 * a - *x;
        Some(*x)
    });

    let cout = &mut create_cout(11 * n);
    cout.add(first_x, ' ');
    cout.add_from_iterator(&mut rest_x_iterator, n - 1, ' ', '\n');
    cout.flush();
}
