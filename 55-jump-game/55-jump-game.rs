impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let length = nums.len();
        let mut can = vec![false; length];
        can[length - 1] = true;
        for i in (0..length).rev() {
            for j in (i + 1)..((i + nums[i] as usize + 1).min(length)) {
                // println!("i: {}, j: {}, can[j]: {}", i, j, can[j]);
                if can[j] {
                    can[i] = true;
                    break;
                }
            }
        }
        can[0]
    }
}
