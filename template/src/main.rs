use std::io::*;

struct CharacterInput<'a> {
    locked_stdin: StdinLock<'a>,
}

#[allow(dead_code)]
impl<'a> CharacterInput<'a> {
    fn scan_u(&mut self) -> usize {
        self.locked_stdin
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

    fn scan_u_pair(&mut self) -> (usize, usize) {
        (self.scan_u(), self.scan_u())
    }

    fn scan_u_triple(&mut self) -> (usize, usize, usize) {
        (self.scan_u(), self.scan_u(), self.scan_u())
    }

    fn scan_s(&mut self) -> String {
        self.locked_stdin
            .by_ref()
            .bytes()
            .map(|byte| byte.unwrap() as char)
            .skip_while(|character| character.is_whitespace())
            .take_while(|character| !character.is_whitespace())
            .collect::<String>()
    }
}

fn create_cin<'a>(input: StdinLock<'a>) -> CharacterInput {
    CharacterInput { locked_stdin: input }
}

struct CharacterOutput {
    string_buffer: String,
}

#[allow(dead_code)]
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

// ------------------------------------------------------------------------

fn process(cin: &mut CharacterInput, cout: &mut CharacterOutput) {
    
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
