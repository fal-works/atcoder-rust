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

    fn scan_u(&mut self, delimiter: u8) -> u64 {
        self.read(delimiter)
            .into_iter()
            .fold(0, |accumulator, byte| {
                (byte - b'0') as u64 + accumulator * 10
            })
    }

    fn scan_u_vec(&mut self, n: usize, separator: u8, delimiter: u8) -> Vec<u64> {
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

fn accumulation(a: &Vec<u64>, dp: &mut Vec<Option<u64>>, end_index: usize) -> u64 {
    match dp[end_index] {
        Some(val) => val,
        None => {
            let acc: u64 = accumulation(a, dp, end_index - 1) + a[end_index - 1];
            dp[end_index] = Some(acc);
            acc
        }
    }
}

fn main() {
    let cin = stdin();
    let cin = &mut create_cin(cin.lock(), 64);

    let n = cin.scan_u(SP) as usize;
    let k = cin.scan_u(LF);
    let a = cin.scan_u_vec(n, SP, LF);

    let mut dp = vec![None as Option<u64>; n + 1];
    dp[0] = Some(0);
    let mut count: u64 = 0;
    let mut last_end_index = 1;

    for start_index in 0..n {
        let prefix_acc = accumulation(&a, &mut dp, start_index);

        for end_index in last_end_index..n + 1 {
            let sub_array_sum = accumulation(&a, &mut dp, end_index) - prefix_acc;
            if sub_array_sum >= k {
                count += 1 + (n - end_index) as u64;
                last_end_index = end_index;
                break;
            } else {
                if end_index == n {
                    println(count);
                    return;
                }
            }
        }
    }

    println(count);
}
