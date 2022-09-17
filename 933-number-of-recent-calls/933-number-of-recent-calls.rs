struct RecentCounter {
    requests: Vec<i32>,
}

impl RecentCounter {
    fn new() -> Self {
        return RecentCounter { requests: vec![] };
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.requests.push(t);
        let bound = t - 3000;
        (self.requests.len() - (match self.requests.binary_search(&bound) {
            Ok(i) => i,
            Err(i) => i,
        })) as i32
    }
}

/*
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */
