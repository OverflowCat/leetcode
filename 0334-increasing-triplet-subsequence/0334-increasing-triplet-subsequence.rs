
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        use std::i32::MAX;
        let (mut min, mut max) = (MAX, MAX);
        for x in nums {
            if x < min {
                min = x;
            } else if x > max {
                return true;
            } else if x != min {
                max = x;
            }
        }
        false
    }
}
