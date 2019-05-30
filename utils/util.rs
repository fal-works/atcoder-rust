fn count_bits(bits: usize, len: usize) -> usize {
    let mut bits = bits;
    (0..len).fold(0, |acc, _| {
        let next = if bits & 1 == 1 { acc + 1 } else { acc };
        bits = bits >> 1;
        next
    })
}
