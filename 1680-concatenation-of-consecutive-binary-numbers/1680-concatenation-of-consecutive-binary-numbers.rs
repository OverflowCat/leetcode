impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        (1..=n as u64).fold(0u64, |o, i| {
            ((o << 64 - i.leading_zeros()) + i) % 1_000_000_007
        }) as i32
    }
}
