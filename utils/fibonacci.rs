const MOD_DIVISOR = 1000000007;

/**
 * Prepare:
 * dp_table[0] = 0;
 * dp_table[1] = 1;
 */
fn fibonacci(index: usize, dp_table: &mut Vec<Option<u32>>) -> u32 {
    dp_table[index].unwrap_or_else(|| {
        let result =
            (fibonacci(index - 2, dp_table) + fibonacci(index - 1, dp_table)) % MOD_DIVISOR;
        dp_table[index] = Some(result);
        result
    })
}
