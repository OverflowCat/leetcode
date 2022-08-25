impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        (nums[nums.len() - 1] - 1) * (nums[nums.len() - 2] - 1)
    }
}
