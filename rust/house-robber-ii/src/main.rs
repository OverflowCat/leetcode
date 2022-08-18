pub fn main() {
    let res = Solution::rob(vec![2i32, 3, 2]);
    assert!(res == 3, "Expected 3, got {}", res);
    println!("{}", res);
    let res = Solution::rob(vec![1, 2i32, 3, 1]);
    assert!(res == 4, "Expected 4, got {}", res);
    println!("{}", res);
    let res = Solution::rob(vec![1, 2, 3]);
    assert!(3 == res, "Expected 3, got {}", res);
    println!("{}", res);
}

struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        {
            let (mut i, mut i1, mut i2) = (0, 0, 0);
            for j in (1..nums.len()).rev() {
                i = i1.max(i2 + nums[j]);
                i2 = i1;
                i1 = i;
            }
            i
        }
        .max({
            let (mut i, mut i1, mut i2) = (0, 0, 0);
            for j in (0..nums.len() - 1).rev() {
                i = i1.max(i2 + nums[j]);
                i2 = i1;
                i1 = i;
            }
            i
        })
    }
}
