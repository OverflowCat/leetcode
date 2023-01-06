impl Solution {
    pub fn max_ice_cream(mut costs: Vec<i32>, mut coins: i32) -> i32 {
        costs.sort_unstable();
        for (i, &x) in costs.iter().enumerate() {
            coins -= x;
            if coins < 0 {
              return i as i32;
          }
        }
        costs.len() as i32
    }
}