fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn number_of_weak_characters(mut properties: Vec<Vec<i32>>) -> i32 {
        properties.sort_unstable_by(|a, b| match b[0].cmp(&a[0]) {
            std::cmp::Ordering::Equal => a[1].cmp(&b[1]),
            other => other,
        });
        /* 3 6, 5 5, 6 3 */
        let mut max_def = 0;
        properties
            .iter()
            .map(|x| {
                if x[1] < max_def {
                    1
                } else {
                    max_def = x[1];
                    0
                }
            })
            .sum()
    }
}
