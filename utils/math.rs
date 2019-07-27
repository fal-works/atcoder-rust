// greatest common divisor
fn gcd(a: u32, b: u32) -> u32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}

// least common multiple
fn lcm(a: u32, b: u32) -> u32 {
    a * b / gcd(a, b)
}

fn average(a: u32, b: u32) -> u32 {
    (a + b) >> 1
}
