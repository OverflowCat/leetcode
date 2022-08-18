impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let memo = &mut Box::new(vec![-1; nums.len()]);
        Self::dp(&nums, 0, memo)
    }

    pub fn dp(nums: &Vec<i32>, start: usize, memo: &mut Box<Vec<i32>>) -> i32 {
        if start >= nums.len() {
            0
        } else if memo[start] != -1 {
            memo[start]
        } else {
            let res =
                Self::dp(nums, start + 1, memo).max(Self::dp(nums, start + 2, memo) + nums[start]);
            memo[start] = res;
            res
        }
    }
}
