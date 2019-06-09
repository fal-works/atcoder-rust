fn createst_common_divisor(a: u32, b: u32) -> u32 {
  if a == 0 { b } else { createst_common_divisor(b % a, a) }
}
