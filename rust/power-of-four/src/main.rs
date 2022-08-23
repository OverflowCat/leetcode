fn main() {
    // let arr: Vec<Option<i32>> = (0..16).map(|x| 4i32.checked_pow(x)).collect();
    // println!("{:?}", arr);
    // let arr: Vec<i32> = (0..16).map(|x| 4i32.pow(x)).collect();
    // println!("{:?}", arr);
    println!("{}", Solution::is_power_of_four(16));
    println!("{}", Solution::is_power_of_four(5));
    println!("{}", Solution::is_power_of_four(1));

}

struct Solution {}

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        const M: [i32; 16] = [
            1, 4, 16, 64, 256, 1024, 4096, 16384, 65536, 262144, 1048576, 4194304, 16777216,
            67108864, 268435456, 1073741824,
        ];
        M.binary_search(&n).is_ok()
    }
}
