use std::collections::HashSet;
use std::iter::FromIterator;

fn hashset(data: &[i32]) -> HashSet<i32> {
    HashSet::from_iter(data.iter().cloned())
}

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let set: HashSet<&i32> = HashSet::from_iter(nums.iter());
        let len = nums.len() as i32;
        for i in 1..=len {
            if !set.contains(&i) {
                return i;
            }
        }
        return len + 1;
    }
}
