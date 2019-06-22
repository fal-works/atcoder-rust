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

#[derive(Copy, Clone)]
struct Task {
    duration: u32,
    limit: u32,
}

fn main() {
    let cin = stdin();
    let cin = &mut create_cin(cin.lock(), 64);

    let n = cin.scan_u(LF) as usize;

    let mut tasks: Vec<Task> = (0..n)
        .map(|_| Task {
            duration: cin.scan_u(SP),
            limit: cin.scan_u(LF),
        })
        .collect();
    tasks.sort_by(|a, b| a.limit.partial_cmp(&b.limit).unwrap());

    let mut accumulated_duration: u32 = 0;

    for task in tasks.into_iter() {
        accumulated_duration += task.duration;
        if accumulated_duration > task.limit {
            println("No");
            return;
        }
    }

    println("Yes");
}
