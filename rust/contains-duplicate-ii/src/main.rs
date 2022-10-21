impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = (1 + k as usize).min(nums.len());
        nums.windows(k)
            .into_iter()
            .any(|s| s.into_iter().collect::<std::collections::HashSet<_>>().len() != k)
    }
}
