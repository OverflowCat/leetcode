impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (mut i, mut i1, mut i2) = (0, 0, 0);
        for j in (0..nums.len()).rev() {
            i = i1.max(i2 + nums[j]);
            i2 = i1;
            i1 = i;
        }
        i
    }
}
