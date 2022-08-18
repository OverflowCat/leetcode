pub fn main() {
    let res = rob(&vec![2, 7, 9, 3, 1], 0);
    println!("{}", res);
    let res = rob(&vec![1, 2, 3, 1], 0);
    println!("{}", res);
    let res = rob(&vec![2, 1, 1, 2], 0);
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

pub fn dp(nums: &Vec<i32>, start: usize) -> i32 {
    if start >= nums.len() {
        return 0;
    }
    return dp(nums, start + 1).max(dp(nums, start + 2) + nums[start]);
}
