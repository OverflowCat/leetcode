impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        for w in nums.windows(3).into_iter().rev() {
            if w[0] + w[1] > w[2] {
                return w[0] + w[1] + w[2];
            }
        }
        0
    }
}