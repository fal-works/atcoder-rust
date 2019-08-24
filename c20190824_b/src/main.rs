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

fn println<T: std::string::ToString>(s: T) {
    println!("{}", s.to_string());
}

// ------------------------------------------------------------------------

fn count_smaller_numbers(slice: &[u32], max_value: u32) -> usize {
    slice.iter().filter(|&&v| v < max_value).count()
}

fn safe_add(a: usize, b: usize) -> usize {
    (a + b) % 1000000007
}

fn create_safe_product_table(base_value: usize, length: usize) -> Vec<usize> {
    let mut table = Vec::<usize>::with_capacity(length + 1);
    table.push(0);

    let mut acc: usize = 0;
    for _ in 0..length {
        acc = safe_add(acc, base_value);
        table.push(acc);
    }

    table
}

fn main() {
    let cin = stdin();
    let cin = &mut create_cin(cin.lock(), 64);

    let n = cin.scan_u(SP) as usize;
    let k = cin.scan_u(LF) as usize;
    let a = cin.scan_u_vec(n, SP, LF);

    let k_triangular = k * (k + 1) / 2;
    let k_triangular_table = create_safe_product_table(k_triangular, n);
    let k_minus_one_triangular = (k - 1) * k / 2;
    let k_minus_one_triangular_table = create_safe_product_table(k_minus_one_triangular, n);

    let count: usize = (0..n).fold(0, |acc, i| {
        let current_value = a[i];
        let former_count = count_smaller_numbers(&a[..i], current_value);
        let latter_count = count_smaller_numbers(&a[i + 1..], current_value);
        let count_to_add = safe_add(
            k_minus_one_triangular_table[former_count],
            k_triangular_table[latter_count],
        );
        safe_add(acc, count_to_add)
    });

    println(count);
}
