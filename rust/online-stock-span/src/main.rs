struct StockSpanner {
    prices: Vec<i32>,
}

impl StockSpanner {
    fn new() -> Self {
        Self {
            prices: Default::default(),
        }
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut acc = 1;
        for i in (0..self.prices.len()).rev() {
            if self.prices[i] < price {
                acc += 1;
            } else {
                break;
            }
        }
        self.prices.push(price);
        acc
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */

fn main() {
    println!("Hello, world!");
}
