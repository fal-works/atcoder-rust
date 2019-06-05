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
    fn add<T: ToString>(&mut self, n: T, separator: char) {
        self.string_buffer += &n.to_string();
        self.string_buffer.push(separator);
    }

    fn flush(&mut self) {
        let out = stdout();
        let mut out = out.lock();
        out.write_all(self.string_buffer.as_bytes()).unwrap();
        self.string_buffer.clear();
    }
}

fn create_cout(buffer_capacity: usize) -> CharacterOutput {
    CharacterOutput {
        string_buffer: String::with_capacity(buffer_capacity),
    }
}

// ------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct RoadworkEvent {
    departure_time: i32,
    position: u32,
    event_type: u8, // 1: start, 0: end
}

fn main() {
    let cin = stdin();
    let cin = &mut create_cin(cin.lock(), 32);

    // read header
    let roadwork_count = cin.scan_u(SP) as usize; // n
    let person_count = cin.scan_u(LF) as usize; // q

    // read roadwork data
    let mut roadwork_events: Vec<RoadworkEvent> = Vec::with_capacity(2 * roadwork_count);

    for _ in 0..roadwork_count {
        let s = cin.scan_u(SP) as i32;
        let t = cin.scan_u(SP) as i32;
        let x = cin.scan_u(LF);

        if t >= x as i32 {
            let departure_time_from = s - x as i32; // because x(t) = t - d for each person
            roadwork_events.push(RoadworkEvent {
                departure_time: departure_time_from,
                event_type: 1,
                position: x,
            });

            let departure_time_to = t - x as i32;
            roadwork_events.push(RoadworkEvent {
                departure_time: departure_time_to,
                event_type: 0,
                position: x,
            });
        }
    }

    roadwork_events.shrink_to_fit();
    roadwork_events.sort_by(|a, b| {
        let time_order = a.departure_time.partial_cmp(&b.departure_time);
        match time_order {
            Some(std::cmp::Ordering::Equal) => a.event_type.partial_cmp(&b.event_type).unwrap(),
            _ => time_order.unwrap(),
        }
    });

    // read departure time data of each person
    let departure_time_list = cin.scan_u_vec(person_count, LF, LF);

    // define how to determine stop position of each person
    let roadwork_events = &roadwork_events;

    struct RoadworkState {
        positions: std::collections::BTreeSet<u32>,
        index: usize,
    }

    let get_stop_position = |state: &mut RoadworkState, person_departure_time: u32| {
        state.index += roadwork_events[state.index..]
            .into_iter()
            .take_while(|event| event.departure_time <= person_departure_time as i32)
            .map(|event| match event.event_type {
                1 => state.positions.insert(event.position),
                0 => state.positions.remove(&event.position),
                _ => false,
            })
            .count();

        let stop_position = state
            .positions
            .iter()
            .map(|n| *n as i32)
            .next() // i.e. the minimum
            .unwrap_or(-1);

        Some(stop_position)
    };

    // output
    let cout = &mut create_cout((6 + 1) * person_count);

    for stop_position in departure_time_list.into_iter().scan(
        RoadworkState {
            positions: std::collections::BTreeSet::new(),
            index: 0,
        },
        get_stop_position,
    ) {
        cout.add(stop_position, '\n');
    }

    cout.flush();
}
