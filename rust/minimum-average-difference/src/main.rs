impl Solution {
    pub fn minimum_average_difference(mut nums: Vec<i32>) -> i32 {
        let mut acc_l = 0;
        let mut acc_r: usize = nums.iter().map(|&x| x as usize).sum();
        let sum = acc_r;
        let mut max_i = 0usize;
        let mut max_diff = usize::MAX;
        nums.pop();
        let n = nums.len();
        nums.into_iter()
            .enumerate()
            .map(|(i, x)| {
                acc_l += x as usize;
                acc_r -= x as usize;
                // print!("{acc_l} / {}, {acc_r} / {} | {i}", i + 1, n - i);
                // Both averages should be rounded down to the nearest integer.
                dbg!(Solution::abs_diff(acc_l / (i + 1), acc_r / (n - i)))
            }) // ↑ LeetCode 是用 debug 模式的，dbg! 不删会超时，真啥b
            .enumerate() // ↓最后一项也要算，此时 acc_r = 0 / 0 = 0
            .chain(std::iter::once((n, sum / (n + 1))))
            .for_each(|(i, diff)| {
                if diff < max_diff {
                    max_diff = diff;
                    max_i = i;
                }
            });
        max_i as i32
    }

    fn abs_diff(a: usize, b: usize) -> usize {
        if a > b {
            a - b
        } else {
            b - a
        }
    }
}

struct Solution {}
fn main() {
    assert_eq!(
        0,
        Solution::minimum_average_difference(vec![0, 1, 0, 1, 0, 1])
    );
    assert_eq!(2, Solution::minimum_average_difference(vec![4, 2, 0]));
}
