use std::collections::HashMap;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut dp = vec![HashMap::new(); nums.len()];
        let mut acc = 0;
        nums.iter().enumerate().for_each(|(i, &x)| {
            nums[..i].iter().enumerate().for_each(|(j, &y)| {
                let diff = x as i64 - y as i64; // [0,2000000000,-294967296]
                *dp[i].entry(diff).or_default() += *dp[j].entry(diff).or_default() + 1;
                acc += dp[j].get(&diff).unwrap();
            });
        });
        acc
    }
}
struct Solution {}
fn main() {
    println!(
        "{}",
        Solution::number_of_arithmetic_slices(vec![2, 4, 6, 8, 10])
    );
    assert_eq!(
        Solution::number_of_arithmetic_slices(vec![0, 2000000000, -294967296]),
        0
    );
}
