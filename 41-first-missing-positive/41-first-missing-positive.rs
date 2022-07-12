use std::collections::HashSet;
use std::iter::FromIterator;

fn hashset(data: &[i32]) -> HashSet<i32> {
    HashSet::from_iter(data.iter().cloned())
}

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let set = hashset(&nums);
        let len = nums.len() as i32;
        for i in 1..=len { // 如果是 len + 1 的话，nums 一定是从 1 到 len 的连续正整数
            if !set.contains(&i) {
                return i;
            }
        }
        return len + 1;
    }
}
