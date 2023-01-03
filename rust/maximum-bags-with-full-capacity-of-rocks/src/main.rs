impl Solution {
    pub fn maximum_bags(mut capacity: Vec<i32>, rocks: Vec<i32>, mut addi: i32) -> i32 {
        rocks
            .into_iter()
            .zip(capacity.iter_mut())
            .for_each(|(r, c)| {
                *c -= r;
            });
        capacity.sort_unstable();
        let mut count = 0;
        for x in capacity {
            addi -= x;
            if addi > -1 {
                count += 1;
            } else {
                break;
            }
        }
        count
    }
}
fn main() {
    println!("Hello, world!");
}
struct Solution;
