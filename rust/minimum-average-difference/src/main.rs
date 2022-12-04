impl Solution {
    pub fn minimum_average_difference(mut nums: Vec<i32>) -> i32 {
        let mut acc_l = 0u64;
        let mut acc_r: u64 = nums.iter().map(|&x| x as u64).sum();
        let mut max_i = 0usize;
        let mut max_diff = f64::MAX;
        nums.pop();
        let n = nums.len();
        nums.into_iter().enumerate().for_each(|(i, x)| {
            acc_l += x as u64;
            acc_r -= x as u64;
            let avg_diff = (acc_l as f64 / (i + 1) as f64 - acc_r as f64 / (n - i) as f64).abs();
            println!("{acc_l} / {}, {acc_r} / {} | {i}: {avg_diff}", i + 1, n - i);
            if avg_diff < max_diff {
                max_diff = avg_diff;
                max_i = i;
            }
        });
        max_i as i32
    }
}

struct Solution {}
fn main() {
    assert_eq!(
        0,
        Solution::minimum_average_difference(vec![0, 1, 0, 1, 0, 1])
    );
}
