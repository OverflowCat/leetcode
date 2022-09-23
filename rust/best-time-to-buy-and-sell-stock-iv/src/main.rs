impl Solution {
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
                let k = k.min(n / 2);
                let mut buy = vec![i32::MIN; k + 1];
                let mut sell = vec![i32::MIN; k + 1];
                buy[0] = -prices[0];
                sell[0] = 0;
                for i in 1..n {
                    let p = prices[i];
                    buy[0] = buy[0].max(sell[0] - p);
                    for j in 1..k + 1 {
                        buy[j] = buy[j].max(sell[j] - p);
                        sell[j] = sell[j].max(buy[j - 1] + p);
                    }
                }
                match sell.iter().max() {
                    Some(&x) => x,
                    None => 0,
                }
            }
        }
    }
}
