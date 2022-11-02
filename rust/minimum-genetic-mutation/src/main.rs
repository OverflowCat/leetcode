#![feature(hash_drain_filter)]
use std::collections::{HashSet, VecDeque};
struct Solution;
impl Solution {
    pub fn min_mutation(mut start: String, end: String, bank: Vec<String>) -> i32 {
        let bank: HashSet<String> = bank.into_iter().collect();
        if !bank.contains(&end) {
            return -1;
        }
        let mut queue = Vec::with_capacity(bank.len());
        while start != end {
            bank.drain_filter(|x| {
                if Solution::diff(x, &start) {
                    queue.push(x.clone());
                    true
                } else {
                    false
                }
            });
            let mut new_queue = Vec::with_capacity(bank.len());
            for x in queue.iter() {
                if Solution::diff(x, &start) {
                    
                }
            }
        }
        -1
    }

    #[inline(always)]
    pub fn diff(a: &String, b: &String) -> bool {
        let mut flag = false;
        for (&x, &y) in a.as_bytes().iter().zip(b.as_bytes().iter()) {
            if x == y {
                if flag {
                    return false;
                }
                flag = true;
            }
        }
        flag
    }
}
