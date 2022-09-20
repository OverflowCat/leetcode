impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (m, n) = (nums1.len(), nums2.len());
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n + 1]; m + 1];
        let mut ans = 0;
        for i in 1..=m {
            for j in 1..=n {
                dp[i][j] = if nums1[i-1] == nums2[j-1] {
                    dp[i-1][j-1] + 1
                } else {
                    0
                };
                ans = ans.max(dp[i][j]);
            }
        }
        ans
    }
}