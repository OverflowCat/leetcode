impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let (mut count, mut max_count) = (0usize, 0usize);
        let v = nums
            .windows(2)
            .map(|slice| slice[0] & slice[1] == 0)
            .collect::<Vec<_>>();
        println!("{:?}", v);

        v.into_iter().for_each(|b| match b {
            true => count += 1,
            false => {
                max_count = max_count.max(count);
                count = 0;
            }
        });
        max_count.max(count).max(1) as i32
    }
}

struct Solution {}

fn main() {
    let nums = vec![1, 3, 8, 48, 10];
    Solution::longest_nice_subarray(nums);
    let nums = vec![
        744437702, 379056602, 145555074, 392756761, 560864007, 934981918, 113312475, 1090, 16384,
        33, 217313281, 117883195, 978927664,
    ];
    Solution::longest_nice_subarray(nums);
}
