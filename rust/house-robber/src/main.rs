pub fn main() {
    let res = rob(vec![2,7,9,3,1]);
    println!("{}", res);
    let res = rob(vec![1,2,3,1]);
    println!("{}", res);
}

pub fn rob(nums: Vec<i32>) -> i32 {
    let length = nums.len();
    if length == 1 {
        return nums[0];
    }
    if length == 2 {
        return nums[0].max(nums[1]);
    }
    // let mut dp: Vec<i32> = Vec::with_capacity(nums.len());
    let mut dp = vec![0; length - 2];
    dp.push(nums[length - 2]);
    dp.push(nums[length - 1]);
    for i in (0..length - 2).rev() {
        dp[i] = dp[i + 1].max(dp[i + 2] + nums[i]);
    }
    // println!("{:?}", dp);
    dp[0].max(dp[1])
}
