fn main() {
    println!("Hello, world!");
}
struct Solution {}
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut count = 0;
        for s in strs {
            count += if s.as_bytes().windows(2).all(|sl| sl[1] > sl[0]) {
                1
            } else {
                0
            };
        }
        count
    }
}
