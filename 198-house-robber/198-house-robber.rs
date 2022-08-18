impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (mut i, mut i1, mut i2) = (0, 0, 0);
        nums.into_iter().rev().for_each(|x| {
            i = i1.max(i2 + x);
            i2 = i1;
            i1 = i;
        });
        i
    }
}
