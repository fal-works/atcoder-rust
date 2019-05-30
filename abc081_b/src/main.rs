use std::io::*;

struct CharacterInput<'a> {
    stdin_ref: StdinLock<'a>,
}

impl<'a> CharacterInput<'a> {
    fn scan_u(&mut self) -> usize {
        self.stdin_ref
            .by_ref()
            .bytes()
            .map(|byte| byte.unwrap() as char)
            .skip_while(|character| character.is_whitespace())
            .take_while(|character| !character.is_whitespace())
            .fold(0, |accumulator, character| {
                (character as u8 - b'0') as usize + accumulator * 10
            })
    }

    fn scan_u_vec(&mut self, n: usize) -> Vec<usize> {
        (0..n).map(|_| self.scan_u()).collect()
    }
}

fn create_cin<'a>(input: StdinLock<'a>) -> CharacterInput {
    CharacterInput { stdin_ref: input }
}

struct CharacterOutput {
    string_buffer: String,
}

impl CharacterOutput {
    fn add<T: ToString>(&mut self, n: T, delimiter: char) {
        self.string_buffer += &n.to_string();
        self.string_buffer.push(delimiter);
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

fn is_odd(n: usize) -> bool {
    n & 1 == 1
}

// ------------------------------------------------------------------------

fn divide_each_by_two(numbers: &mut Vec<usize>, length: usize) -> std::result::Result<(), usize> {
    for i in 0..length {
        if is_odd(numbers[i]) {
            return Err(i);
        }

        numbers[i] >>= 1;
    }

    Ok(())
}

fn process(cin: &mut CharacterInput, cout: &mut CharacterOutput) {
    let n = cin.scan_u();
    let a = &mut cin.scan_u_vec(n);
    let mut count = 0;

    while divide_each_by_two(a, n).is_ok() {
        count += 1;
    }

    cout.add(count, '\n');
}

// ------------------------------------------------------------------------

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let cin = &mut create_cin(cin);
    let cout = &mut create_cout(0);

    process(cin, cout);

    cout.flush();
}
