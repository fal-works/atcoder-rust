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
}

fn create_cin<'a>(input: StdinLock<'a>) -> CharacterInput {
    CharacterInput {
        locked_stdin: input,
        bytes_buffer: Vec::with_capacity(1000),
    }
}

fn println(s: &str) {
    let stdout = stdout();
    let mut locked_stdout = stdout.lock();
    locked_stdout.write_all(s.as_bytes()).unwrap();
    locked_stdout.write_all(b"\n").unwrap();
    locked_stdout.flush().unwrap();
}

// ------------------------------------------------------------------------

trait Parity {
    fn is_even(self) -> bool;
    fn is_odd(self) -> bool;
    fn has_same_parity<T: Parity>(self, other: T) -> bool;
}

macro_rules! impl_parity {
    ($($t:ty),+) => {
        $(impl Parity for $t {
            fn is_even(self) -> bool {
                self & 1 == 0
            }
            fn is_odd(self) -> bool {
                self & 1 == 1
            }
            fn has_same_parity<T: Parity>(self, other: T) -> bool {
                self.is_even() == other.is_even()
            }
        })+
    }
}

impl_parity!(u32, i32);

// ------------------------------------------------------------------------

#[derive(Copy, Clone)]
struct Point {
    t: i32,
    x: i32,
    y: i32,
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            t: self.t - other.t,
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Point {
    fn new() -> Point {
        Point { t: 0, x: 0, y: 0 }
    }

    fn delta_time_from(self, other: Point) -> i32 {
        self.t - other.t
    }

    fn distance_from(self, other: Point) -> u32 {
        (self.x - other.x).abs() as u32 + (self.y - other.y).abs() as u32
    }

    fn is_reachable_from(self, other: Point) -> bool {
        let delta_time = self.delta_time_from(other) as u32; // assuming positive
        let distance = self.distance_from(other);

        delta_time >= distance && delta_time.has_same_parity(distance)
    }
}

fn scan_point(cin: &mut CharacterInput) -> Point {
    Point {
        t: cin.scan_u(SP) as i32,
        x: cin.scan_u(SP) as i32,
        y: cin.scan_u(LF) as i32,
    }
}

// ------------------------------------------------------------------------

fn process(cin: &mut CharacterInput) {
    let n = cin.scan_u(LF);

    let any_unreachable = (0..n)
        .map(|_| scan_point(cin))
        .scan(Point::new(), |previous_point, current_point| {
            let reachable = current_point.is_reachable_from(*previous_point);
            *previous_point = current_point;
            Some(reachable)
        })
        .any(|reachable| reachable == false);

    println(
        if any_unreachable {
            "No"
        } else {
            "Yes"
        }
    );
}

// ------------------------------------------------------------------------

fn main() {
    let cin = stdin();
    let cin = &mut create_cin(cin.lock());

    process(cin);
}
