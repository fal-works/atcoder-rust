trait StepSum: Iterator {
    fn step_sum(&mut self, step: usize) -> u32;
}

impl<I: Iterator> StepSum for I where I: Iterator<Item = u32> {
    fn step_sum(&mut self, step: usize) -> u32 {
        let mut acc;

        match self.next() {
            Some(value) => acc = value,
            None => return 0
        }

        loop {
            match self.nth(step - 1) {
                Some(value) => acc += value,
                None => break
            }
        }

        acc
    }    
}
