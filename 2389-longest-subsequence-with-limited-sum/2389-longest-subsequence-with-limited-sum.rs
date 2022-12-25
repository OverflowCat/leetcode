impl Solution {
    pub fn answer_queries(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        let mut sum: u64 = 0;
        let sums: Vec<u64> = nums
            .iter()
            .map(|&x| {
                sum += x as u64;
                sum
            })
            .collect();
        queries
            .into_iter()
            .map(|q| {
                (match sums.binary_search(&(q as u64)) {
                    Ok(x) => x + 1,
                    Err(x) => x,
                }) as i32
            })
            .collect()
    }
}