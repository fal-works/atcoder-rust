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

    fn scan_s(&mut self, delimiter: u8) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.read(delimiter)) }
    }

    fn scan_i(&mut self, delimiter: u8) -> i32 {
        self.scan_s(delimiter).parse().unwrap()
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
 
type Pos = (i32, i32);
 
use std::collections::HashMap;
 
fn main() {
    let cin = stdin();
    let cin = &mut create_cin(cin.lock(), 32);
 
    let n = cin.scan_u(LF) as usize;

    if n <= 2 {
        println(1);
        return;
    }

    let ball_list: Vec<Pos> = (0..n).map(|_| (cin.scan_i(SP), cin.scan_i(LF))).collect();
 
    let mut pos_diff_count_map: HashMap<Pos, usize> = HashMap::with_capacity(n * n);
 
    for i in 0..n - 1 {
        let ball_a = ball_list[i];
 
        for k in i + 1..n {
            let ball_b = ball_list[k];
            let pos_diff = if ball_b.0 > ball_a.0 || (ball_b.0 == ball_a.0 && ball_b.1 > ball_a.1) {
                (ball_b.0 - ball_a.0, ball_b.1 - ball_a.1)
            } else {
                (ball_a.0 - ball_b.0, ball_a.1 - ball_b.1)
            };
 
            let count = pos_diff_count_map.entry(pos_diff).or_insert(0);
            *count += 1;
        }
    }
 
    let max_count = *pos_diff_count_map.values().max().unwrap();
 
    println(n - max_count);
}
