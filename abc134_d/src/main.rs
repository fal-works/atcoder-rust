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

fn main() {
    let cin = stdin();
    let cin = &mut create_cin(cin.lock(), 2 * 200000);

    let n = cin.scan_u(LF) as usize;
    let a_bytes = cin.read(LF);
    let mut k = 0;
    let a: Vec<bool> = (0..n)
        .map(|_| {
            let flag = a_bytes[k] == b'1';
            k += 2;
            flag
        })
        .collect();

    let mut ball: Vec<bool> = vec![false; n];
    let mut box_num_list = Vec::<usize>::with_capacity(n);

    for i in (0..n).rev() {
        let box_num = i + 1;

        let mut acc = ball[i];
        let mut k = i + box_num;
        while k < n {
            acc ^= ball[k];
            k += box_num;
        }

        if acc != a[i] {
            ball[i] = true;
            box_num_list.push(box_num);
        }
    }

    let box_len = box_num_list.len();
    let cout = &mut create_cout(7 * box_len);

    cout.add(box_len, '\n');

    if box_len > 0 {
        cout.add_from_iterator(&mut box_num_list.iter(), box_len, ' ', '\n');
    }

    cout.flush();
}
