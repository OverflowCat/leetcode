use std::collections::{HashMap, HashSet};

fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut pairs: HashMap<String, usize> = HashMap::with_capacity(words.len() / 2);
        words.into_iter().for_each(|w| {
            let rev = w.chars().rev().collect::<String>();
            let a = pairs.entry(w);
            let b = pairs.entry(rev);
            
        });
        114
    }
}
