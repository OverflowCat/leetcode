struct MedianFinder {
    data: Vec<i32>,
    is_sorted: bool,
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            data: Default::default(),
            is_sorted: false,
        }
    }

    fn add_num(&mut self, num: i32) {
        self.data.push(num);
        self.is_sorted = false;
    }

    fn find_median(&mut self) -> f64 {
        if !self.is_sorted {
            self.data.sort();
            self.is_sorted = true;
        }
        let length = self.data.len();
        // length > 0 is guaranteed
        let half = length / 2;
        if length % 2 == 0 {
            // length >= 2
            (self.data[half - 1] as f64 + self.data[half] as f64) / 2.
        } else {
            self.data[half] as f64
        }
    }
}

/*
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
