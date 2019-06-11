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
