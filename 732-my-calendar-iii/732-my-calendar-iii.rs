use std::collections::BTreeMap;

#[derive(Default)]
struct MyCalendarThree {
    data: BTreeMap<i32, i32>,
}

impl MyCalendarThree {
    fn new() -> Self {
        Default::default()
    }

    fn book(&mut self, start: i32, end: i32) -> i32 {
        *self.data.entry(start).or_insert(0) += 1;
        *self.data.entry(end).or_insert(0) -= 1;
        let mut curr = 0;
        let mut res = 0;
        self.data.iter().for_each(|(_, &x)| {
            curr += x;
            res = curr.max(res);
        });
        res
    }
}
