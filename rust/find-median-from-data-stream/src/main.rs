use std::collections::BinaryHeap;

struct MedianFinder {
    data: BinaryHeap<i32>,
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            data: Default::default(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.data.push(num);
    }

    fn find_median(&mut self) -> f64 {
        let length = self.data.len();
        // length > 0 is guaranteed
        let half = length / 2;
        if length % 2 == 0 {
            // length >= 2
            let d = self.data.iter();
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
