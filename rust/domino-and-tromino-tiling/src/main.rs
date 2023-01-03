use std::vec;

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        if n < 3 {
            n
        } else {
            let n = n as usize;
            let mut dp1 = Vec::with_capacity(n + 1);
            dp1.push(0u64);
            dp1.push(1);
            dp1.push(2);
            let mut dp2 = vec![0, 0, 2u64];
            for i in 3..=n {
                dp1.push((dp1[i - 1] + dp1[i - 2] + dp2[i - 1]) % 1000000007);
                dp2.push((2 * dp2[i - 2] + dp2[i - 1]) % 1000000007);
            }
            (dp1[n] % 1000000007) as i32
        }
    }
}
struct Solution;

fn main() {
    println!("{}", Solution::num_tilings(5));
}
