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
