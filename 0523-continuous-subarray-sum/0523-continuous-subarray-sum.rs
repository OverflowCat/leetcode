use std::collections::HashMap;
impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() >= 2 {
            let mut hm: HashMap<i32, usize> = HashMap::new();
            hm.insert(0, usize::MAX);
            let mut remainder = 0;
            for (i, x) in nums.into_iter().enumerate() {
                remainder = (remainder + x) % k;
                if i - *hm.entry(remainder).or_insert(i) > 1 {
                    return true;
                }
            }
        }
        false
    }
}