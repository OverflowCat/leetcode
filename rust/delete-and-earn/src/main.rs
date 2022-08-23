fn main() {
    let res = Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4]);
    assert!(res == 9, "Expected 9, got {}", res);
}

struct Solution {}

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut count = [0usize; 10001];
        let mut max = 0usize;
        for &num in nums.iter() {
            let num = num as usize;
            count[num] += num;
            max = max.max(num);
        }
        for i in 2..=max {
            count[i] = count[i - 1].max(count[i - 2] + count[i]);
        }
        count[max] as i32
    }
}
