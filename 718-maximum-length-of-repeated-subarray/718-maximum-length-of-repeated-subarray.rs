impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (m, n) = (nums1.len(), nums2.len());
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n + 1]; m + 1];
        let mut ans = 0;
        for (i, &x) in nums1.iter().enumerate() {
            for (j, &y) in nums2.iter().enumerate() {
                let v = if x == y {
                    dp[i][j] + 1
                } else {
                    0
                };
                dp[i + 1][j + 1] = v;
                ans = ans.max(v);
            }
        }
        ans
    }
}