fn main() {
    println!("Hello, world!");
}
struct Solution {}

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let (mut min_v, mut min_i, mut cur): (i32, _, _) = Default::default();
        gas.into_iter()
            .zip(cost.into_iter())
            .enumerate()
            .for_each(|(i, (gas, cost))| {
                cur += gas - cost;
                if cur < min_v {
                    min_v = cur;
                    min_i = i + 1;
                }
            });
        if cur < 0 {
            -1
        } else {
            min_i as i32
        }
    }
}
