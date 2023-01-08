fn main() {
    println!("Hello, world!");
}
struct Solution {}

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        s.split(' ').count() == pattern.as_bytes().len() && {
            let s1: std::collections::HashSet<_> = s.split(' ').collect();
            let mut s2 = [0u16; 26];
            pattern.as_bytes().iter().for_each(|&c| {
                s2[(c - b'a') as usize] += 1;
            });
            let n2: u8 = s2.into_iter().map(|&x| if x == 0 { 0 } else { 1u8 }).sum();
            let n1 = s1.len();
            let n3 = s1.into_iter().zip(s.as_bytes().into_iter()).count();
            n1 == n2 as usize && n1 == n3
        }
    }
}
