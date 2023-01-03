use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}
struct Solution {}

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        // let s1: HashSet<u8> = pattern.into_bytes().into_iter().collect();
        // let s2: HashSet<_> = s.split(' ').collect();
        let s1 = pattern.into_bytes();
        let mut n2 = 0;
        let s2: Vec<_> = s
            .split(' ')
            .map(|x| {
                n2 += 1;
                x
            })
            .collect();
            let shorter: &Vec<_>;
            let longer: &Vec<_>;
            if n2 < n1.len() {
                
            }

        }
    }
}
