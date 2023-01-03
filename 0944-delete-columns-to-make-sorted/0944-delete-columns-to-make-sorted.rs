impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        (0..strs[0].len())
            .map(|i| {
                if strs
                    .windows(2)
                    .any(|pair| pair[0].as_bytes()[i] > pair[1].as_bytes()[i])
                {
                    1
                } else {
                    0
                }
            })
            .sum()
    }
}
