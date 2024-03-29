fn main() {
    println!("{}", Solution::concatenated_binary(3));
}

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        let mut o = 0;
        for i in 1..=n as u64 {
            o <<= 64 - i.leading_zeros();
            o += i;
            o %= 1_000_000_007
        }
        o as i32
    }
}

struct Solution;
