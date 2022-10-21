use std::collections::HashSet;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut seen: HashSet<i32> = HashSet::new();
        let k = k as usize;
        nums.iter().enumerate().any(|(i, x)| {
            if seen.contains(x) {
                true
            } else {
                seen.insert(*x);
                if seen.len() > k {
                    seen.remove(&nums[i - k]);
                }
                false
            }
        })
    }
}