impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        plant_time
            .into_iter()
            .zip(grow_time.into_iter())
            .collect::<std::collections::BinaryHeap<_>>()
            .into_iter()
            .for_each(|x| println!("{:?}", x));
        2
    }
}

struct Solution {}

fn main() {
    let res = Solution::earliest_full_bloom(vec![1, 4, 3,2,2], vec![2, 3, 1,6,5]);
}
