fn main() {
    println!("Hello, world!");
}

struct Solution {}

pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
    let n = prices.len();
    let k = k as usize;
    match n {
        _ if n <= 1 => 0,
        n if n >= n / 2 => prices
            .windows(2)
            .map(|x| if x[1] > x[0] { x[1] - x[0] } else { 0 })
            .sum(),
        n => {
            let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; k + 1];
            for i in 1..=k {
                let mut local_max = dp[i - 1][0] - prices[0];
                for j in 1..=n {
                    dp[i][j] = dp[i][j - 1].max(prices[j] + local_max);
                    local_max = local_max.max(dp[i - 1][j] - prices[j]);
                }
            }
            dp[k][n - 1]
        }
    }
}