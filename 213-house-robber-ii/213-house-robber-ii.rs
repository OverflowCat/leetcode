impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.len() {
            1 => nums[0],
            2 => nums[0].max(nums[1]),
            x => {
                let (mut i, mut i1, mut i2) = (0, 0, 0);
                for j in (1..x).rev() {
                    i = i1.max(i2 + nums[j]);
                    i2 = i1;
                    i1 = i;
                }
                i
            }
            .max({
                let (mut i, mut i1, mut i2) = (0, 0, 0);
                for j in (0..x - 1).rev() {
                    i = i1.max(i2 + nums[j]);
                    i2 = i1;
                    i1 = i;
                }
                i
            }),
        }
    }
}
