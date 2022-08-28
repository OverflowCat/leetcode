impl Solution {
    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let length = nums.len();
        nums.sort_unstable();
        let mut sum = 0;
        let mut nums_i = 0;
        queries
            .iter()
            .map(|&q| {
                if nums_i == length - 1 {
                    nums_i as i32 + 1
                } else {
                    while sum <= q && nums_i < length {
                        sum += nums[nums_i];
                        nums_i += 1;
                    }
                    if sum > q {
                        nums_i as i32 - 1
                    } else {
                        nums_i as i32
                    }
                }
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
}
