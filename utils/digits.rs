struct DigitIterator {
    value: usize,
    base: usize,
}

impl DigitIterator {
    fn new<T: Into<usize>>(value: T, base: usize) -> DigitIterator {
        assert!(base > 0);
        DigitIterator {
            value: value.into(),
            base: base,
        }
    }
}

impl Iterator for DigitIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value > 0 {
            let digit = self.value % self.base;
            self.value /= self.base;
            Some(digit)
        } else {
            None
        }
    }
}

trait Digits {
    fn digits(self, base: usize) -> DigitIterator;
}

impl <T: Into<usize>> Digits for T {
    fn digits(self, base: usize) -> DigitIterator {
        DigitIterator::new(self, base)
    }
}
