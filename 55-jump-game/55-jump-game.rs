impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let length = nums.len();
        let mut min_accessible = length - 1;
        nums.into_iter().enumerate().rev().for_each(|(i, num)| {
            // println!("{} {}", i, num);
            if i + num as usize >= min_accessible {
                min_accessible = i;
            }
        });
        min_accessible == 0
    }
}
