impl Solution {
    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut sum = 0;
        let sums: Vec<i32> = nums
            .iter()
            .map(|&x| {
                sum += x;
                sum
            })
            .collect();
        queries
            .iter()
            .map(|q| {
                (match sums.binary_search(&(*q + 1)) {
                    Ok(x) => x - 1,
                    Err(x) => x,
                }) as i32
            })
            .collect()
    }
}

struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::answer_queries(vec![4, 5, 2, 1], vec![3, 10, 21])
    );
    println!(
        "{:?}",
        Solution::answer_queries(vec![3,4,5,2], vec![1])
    );
}
