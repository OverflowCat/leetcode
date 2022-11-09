impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let (mut left, mut max_len) = (0usize, 0usize);
        for i in 1..nums.len() {
            for j in left..i {
                if nums[j] & nums[i] != 0 {
                    max_len = max_len.max(i - left);
                    left = j + 1;
                    break;
                }
            }
        }
        max_len.max(nums.len() - 1 - left) as i32
    }
}

struct Solution {}

fn main() {
    let nums = vec![1, 3, 8, 48, 10];
    let res = Solution::longest_nice_subarray(nums);
    println!("{}", res);
    let nums = vec![
        744437702, 379056602, 145555074, 392756761, 560864007, 934981918, 113312475, 1090, 16384,
        33, 217313281, 117883195, 978927664,
    ];
    let res = Solution::longest_nice_subarray(nums);
    println!("{}", res);
    let nums = vec![
        84139415, 693324769, 614626365, 497710833, 615598711, 264, 65552, 50331652, 1, 1048576,
        16384, 544, 270532608, 151813349, 221976871, 678178917, 845710321, 751376227, 331656525,
        739558112, 267703680,
    ];
    let res = Solution::longest_nice_subarray(nums);
    println!("{}", res);
}
