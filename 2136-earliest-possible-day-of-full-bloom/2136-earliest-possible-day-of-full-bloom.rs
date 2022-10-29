impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let mut a = plant_time
            .into_iter()
            .zip(grow_time.into_iter())
            .collect::<Vec<_>>();
        a.sort_unstable_by_key(|v| -v.1);
        let (mut sow, mut bloom) = (0, 0);
        for (p, g) in a {
            sow += p;
            bloom = bloom.max(g + sow);
        }
        bloom
    }
}