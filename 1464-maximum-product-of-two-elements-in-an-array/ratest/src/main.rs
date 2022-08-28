impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut 最大 = 0;
        let mut 第二大 = 0; // [10,2,5,2]
        nums.into_iter().for_each(|x| {
            if x >= 第二大 {
                if x >= 最大 {
                    第二大 = 最大;
                    最大 = x;
                } else {
                    第二大 = x;
                }
            }
        });
        (最大 - 1) * (第二大 - 1)
    }
}
