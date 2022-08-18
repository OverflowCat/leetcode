pub fn main() {
    let res = Solution::rob(vec![2i32, 7, 9, 3, 1]);
    println!("{}", res);
    let res = Solution::rob(vec![1, 2i32, 3, 1]);
    println!("{}", res);
    let res = Solution::rob(vec![2i32, 1, 1, 2]);
    println!("{}", res);
    /*
    59 / 68 test cases passed.
    Input:
    [2,1,1,2]
    Output:
    3
    Expected:

    4
    */
}

struct Solution {}

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
