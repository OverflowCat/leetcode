impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        dp[1] = 1;
        for i in 2..=n {
            let mut min = i32::MAX;
            for ji in 1..i32::MAX
            {
                let j = ji.pow(2);
                if j > i { break; }
                min = dp[(i - j) as usize].min(min);
            }
            dp[i as usize] = min + 1;
        }
        dp[n as usize]
    }
}