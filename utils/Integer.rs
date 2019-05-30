trait Integer {
    fn is_even(self) -> bool;
    fn is_odd(self) -> bool;
}

impl <T: Into<usize>> Integer for T {
    fn is_even(self) -> bool {
        self.into() & 1 == 0
    }
    fn is_odd(self) -> bool {
        self.into() & 1 == 1
    }
}
