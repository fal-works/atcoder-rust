trait Matrixify<T> {
    fn columns(self, count: usize, estimated_row_count: usize) -> Vec<Vec<T>>;
}

impl<I, T> Matrixify<T> for I
where
    I: Iterator<Item = T>,
{
    fn columns(self, column_count: usize, estimated_row_count: usize) -> Vec<Vec<T>> {
        assert!(column_count > 0);
        let mut columns: Vec<Vec<T>> = (0..column_count)
            .map(|_| Vec::with_capacity(estimated_row_count))
            .collect();
        let mut column_index = 0;

        for elm in self.into_iter() {
            columns[column_index].push(elm);
            column_index = (column_index + 1) % column_count;
        }

        columns
    }
}
