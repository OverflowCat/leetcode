impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        (nums
            .windows(2)
            .map(|slice| slice[0] + slice[1])
            .collect::<std::collections::HashSet<i32>>())
        .len()
            != nums.len() - 1
    }
}
