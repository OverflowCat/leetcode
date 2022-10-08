use std::{cmp::Ordering::*, i32::MAX};
impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let last_i = nums.len() - 1;
        let (mut ans, mut min_diff) = (nums[0] + nums[1] + nums[2], MAX);
        for (i, &x) in nums.iter().enumerate() {
            let (mut ini, mut end) = (i + 1, last_i);
            while ini < end {
                let sum = x + nums[ini] + nums[end];
                let diff = (sum - target).abs();
                if diff < min_diff {
                    ans = sum;
                    min_diff = diff;
                }
                match sum.cmp(&target) {
                    Equal => {
                        return sum;
                    }
                    Greater => {
                        if end > 0 {
                            end -= 1;
                        }
                    }
                    Less => {
                        if ini < last_i {
                            ini += 1;
                        }
                    }
                };
            }
        }
        ans
    }
}