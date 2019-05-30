trait CheckRange<T: std::cmp::PartialOrd> {
    /** Naive alternative to contains(), which is not supported by v1.15.1 */
    fn includes(&self, n: T) -> bool;
}

impl <T: std::cmp::PartialOrd> CheckRange<T> for std::ops::Range<T> {
    fn includes(&self, n: T) -> bool {
        self.start <= n && n < self.end
    }
}
