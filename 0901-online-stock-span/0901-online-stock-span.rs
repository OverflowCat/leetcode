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
            if self.prices[i] > price {
                break;
            } else {
                acc += 1;
            }
        }
        self.prices.push(price);
        acc
    }
}