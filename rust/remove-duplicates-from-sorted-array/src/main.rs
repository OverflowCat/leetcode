use std::collections::HashSet;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut count = 0;
        let mut set = HashSet::new();
        let mut j = 0;
        for i in 0..nums.len() {
            if set.insert(nums[i]) {
                nums[j] = nums[i];
                j += 1;
                count += 1;
            }
        }
        count
    }
}

struct Solution;

fn main() {
    let mut a = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let c = Solution::remove_duplicates(&mut a);
    println!("{}, {:?}", c, a);
}
