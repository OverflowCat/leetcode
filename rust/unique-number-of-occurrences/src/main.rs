use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut count: HashMap<i32, usize> = HashMap::new();
        arr.into_iter().for_each(|x| {
            *count.entry(x).or_default() += 1;
        });
        let mut set = HashSet::new();
        for v in count.values() {
            if !set.insert(v) {
                return false;
            }
        }
        true
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
