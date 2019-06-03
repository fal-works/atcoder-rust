fn count_consecutive_bytes(bytes: &Vec<u8>, start_index: usize, end_index: usize) -> usize {
    let byte_to_count = bytes[start_index];
    let mut count = 1;

    for index in start_index + 1..end_index {
        if bytes[index] != byte_to_count {
            break;
        }

        count += 1;
    }

    count
}

#[derive(Copy, Clone)]
struct ConsecutiveBytes {
    unit_byte: u8,
    count: usize,
}

impl ConsecutiveBytes {
    fn new(unit_byte: u8) -> ConsecutiveBytes {
        ConsecutiveBytes {
            unit_byte: unit_byte,
            count: 0,
        }
    }

    fn count(&mut self, byte: u8) -> usize {
        if byte == self.unit_byte {
            self.count += 1;
        } else {
            self.count = 0;
        }

        self.count
    }
}
