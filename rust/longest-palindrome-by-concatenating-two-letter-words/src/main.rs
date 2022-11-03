use std::collections::{HashMap, HashSet};

fn main() {
    assert_eq!(
        6,
        Solution::longest_palindrome(vec![
            "ab".into(),
            "ty".into(),
            "yt".into(),
            "lc".into(),
            "cl".into(),
            "ab".into()
        ])
    );
}

struct Solution {}

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut hm: HashMap<String, usize> = HashMap::with_capacity(words.len());
        let mut counter = 0;
        let mut is_middle_not_used = true;
        words.into_iter().for_each(|w| {
            let rev = w.chars().rev().collect::<String>();
            if is_middle_not_used && rev == w {
                counter += w.len();
                is_middle_not_used = true;
                return;
            }
            let mut flag = 0u8;
            hm.entry(rev.clone()).and_modify(|count| {
                if *count == 1 {
                    flag = 2;
                } else {
                    *count -= 1;
                    flag = 1;
                }
                counter += rev.len();
            });
            match flag {
                2 => {
                    *hm.entry(rev).or_insert(0) += 1;
                }
                0 => {
                    *hm.entry(w).or_insert(0) += 1;
                }
                _ => {}
            }
        });
        counter as i32
    }
}
