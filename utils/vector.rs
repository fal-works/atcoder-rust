fn create_populated_vec<T, F>(capacity: usize, factory: F) -> Vec<T>
where
    F: Fn(usize) -> T,
{
    let mut vector: Vec<T> = Vec::with_capacity(capacity);
    for i in 0..capacity {
        vector.push(factory(i));
    }
    vector
}
