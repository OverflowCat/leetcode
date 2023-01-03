impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut dp: Vec<(i32, i32)> = vec![Default::default(); n];
        dp[0].1 = -prices[0];
        if n > 1 {
            dp[1] = (dp[0].0.max(dp[0].1 + prices[1]), dp[0].1.max(-prices[1]));
            prices.into_iter().enumerate().skip(2).for_each(|(i, p)| {
                dp[i] = (
                    dp[i - 1].0.max(dp[i - 1].1 + p),
                    dp[i - 1].1.max(dp[i - 2].0 - p),
                );
            });
        }
        dp.last().unwrap().0
    }
}
fn main() {}
struct Solution {}
