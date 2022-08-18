impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        start_time
            .into_iter()
            .zip(end_time.into_iter())
            .map(|(start, end)| {
                if start <= query_time && query_time <= end {
                    1
                } else {
                    0
                }
            })
            .sum()
    }
}