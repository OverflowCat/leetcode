impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut min_accessible = nums.len() - 1;
        nums.into_iter().enumerate().rev().for_each(|(i, num)| {
            // println!("{} {}", i, num);
            if i + num as usize >= min_accessible {
                min_accessible = i;
            }
        });
        min_accessible == 0
    }
}
