impl Solution {
    pub fn sum_even_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut even = nums.iter().map(|&i| if i % 2 == 0 {i} else {0}).sum();
        queries.iter().map(|x| {
            let (v, i) = (x[0], x[1] as usize);
            if nums[i] % 2 == 0 { even -= nums[i]; }
            let w = nums[i] + v; if w % 2 == 0 { even += w; }
            nums[i] += v;
            even
        }).collect()
    }
}
