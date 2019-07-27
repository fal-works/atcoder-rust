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

    fn scan_u(&mut self, delimiter: u8) -> u64 {
        self.read(delimiter)
            .into_iter()
            .fold(0, |accumulator, byte| {
                (byte - b'0') as u64 + accumulator * 10
            })
    }

    fn scan_u_vec(&mut self, n: usize, separator: u8, delimiter: u8) -> Vec<u64> {
        let mut vector = Vec::with_capacity(n);
        if n > 1 {
            for _i in 0..n - 1 {
                vector.push(self.scan_u(separator));
            }
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


// ------------------------------------------------------------------------

fn main() {
    let cin = stdin();
    let cin = &mut create_cin(cin.lock(), 16);

    let n = cin.scan_u(LF) as usize;
    let mut a = cin.scan_u_vec(n + 1, SP, LF);
    let b = cin.scan_u_vec(n, SP, LF);

    let total_count = (0..n).map(|i| {
        let monster_count = a[i] + a[i + 1];
        let max_defeatable_count = b[i];

        if max_defeatable_count >= monster_count {
            a[i + 1] = 0;
            return monster_count;
        }

        if max_defeatable_count > a[i] {
            a[i + 1] = monster_count - max_defeatable_count;
        }

        return max_defeatable_count;
    }).sum::<u64>();

    println!("{}", total_count);
}
