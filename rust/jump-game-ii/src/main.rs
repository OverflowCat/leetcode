fn main() {
    println!("{}", Solution::can_jump(vec![2, 3, 1, 1, 4])); // true
    println!("{}", Solution::can_jump(vec![3, 2, 1, 0, 4])); // false
    println!("{}", Solution::can_jump(vec![3])); // true
    println!("{}", Solution::can_jump(vec![0, 0])); // false
    println!("{}", Solution::can_jump(vec![0])); // true
}

struct Solution {}

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
