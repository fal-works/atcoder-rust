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

struct Grid<T> {
    vec: Vec<T>,
    row_count: usize,
    column_count: usize
}

impl<T> Grid<T>
where
    T: Clone + Copy,
{
    fn new(initial_value: T, column_count: usize, row_count: usize) -> Grid<T> {
        Grid {
            vec: vec![initial_value; column_count * row_count],
            row_count: row_count,
            column_count: column_count
        }
    }
}

use std::ops::{Index, IndexMut};

impl<T> Index<usize> for Grid<T> {
    type Output = [T];
    fn index(&self, row: usize) -> &[T] {
        debug_assert!(row < self.row_count);
        let start_index = row * self.column_count;
        &self.vec[start_index..start_index + self.column_count]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, row: usize) -> &mut [T] {
        debug_assert!(row < self.row_count);
        let start_index = row * self.column_count;
        &mut self.vec[start_index..start_index + self.column_count]
    }
}


trait ExIterator: Iterator {
    fn foreach<F>(self, mut callback: F)
    where
        Self: Sized,
        F: FnMut(Self::Item),
    {
        self.fold((), move |(), item| callback(item));
    }
}

impl<I: Iterator> ExIterator for I {}


trait GridReader<T> {
    fn scan_grid(
        &mut self,
        width: usize,
        height: usize,
        initial_value: T,
        key_value_arrays: &[(u8, T)],
    ) -> Grid<T>;
}

impl<'a, T> GridReader<T> for CharacterInput<'a>
where
    T: Clone + Copy,
{
    /**
     * Example:
     * let mut grid = cin.scan_grid(w, h, 0 as u32, &[(b'#', 1)]);
     */
    fn scan_grid(
        &mut self,
        column_count: usize,
        row_count: usize,
        initial_value: T,
        key_value_arrays: &[(u8, T)],
    ) -> Grid<T> {
        let mut grid = Grid::new(initial_value, column_count, row_count);

        for row_index in 0..row_count {
            let byte_row = self.read(LF);
            let grid_row = &mut grid[row_index];

            (0..column_count)
                .filter_map(|x| {
                    key_value_arrays
                        .into_iter()
                        .filter(|entry| byte_row[x] == entry.0)
                        .map(|entry| (x, entry.1))
                        .next()
                })
                .foreach(|(x, value)| grid_row[x] = value);
        }

        grid
    }
}

// ------------------------------------------------------------------------

fn main() {
    let cin = stdin();
    let cin = &mut create_cin(cin.lock(), 32);

    let h = cin.scan_u(SP) as usize;
    let w = cin.scan_u(LF) as usize;

    let mut grid = cin.scan_grid(w, h, 0 as i32, &[(b'#', -1)]);

    for y in 0..h {
        let row = &mut grid[y];
        let mut x = 0;
        while x < w {
            if row[x] != -1 {
                let mut next_x = x + 1;
                while next_x < w && row[next_x] != -1 {
                    next_x += 1;
                }
                let count = next_x - x;

                for x in x..next_x {
                    row[x] = count as i32;
                }
                x = next_x + 1;
            } else {
                x += 1;
            }
        }
    }

    let mut max_count = -1;

    for x in 0..w {
        let mut y = 0;
        while y < h {
            if grid[y][x] != -1 {
                let mut next_y = y + 1;
                while next_y < h && grid[next_y][x] != -1 {
                    next_y += 1;
                }
                let count = next_y - y;

                for y in y..next_y {
                    let mut row = &mut grid[y];
                    row[x] += count as i32;
                    if row[x] > max_count { max_count = row[x]; }
                }

                y = next_y + 1;
            } else {
                y += 1;
            }
        }
    }

    println(max_count - 1);
}
