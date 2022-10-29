impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        plant_time
            .into_iter()
            .zip(grow_time.into_iter())
            .collect::<std::collections::BinaryHeap<_>>()
            .into_iter()
            .fold((0, 0), |(sow, bloom), (plant, grow)| {
                (sow + plant, bloom.max(sow + grow))
            })
            .1
    }
}

struct Solution {}

