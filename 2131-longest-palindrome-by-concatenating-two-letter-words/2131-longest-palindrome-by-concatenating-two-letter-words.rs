use std::collections::BTreeMap;

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut palindromes: BTreeMap<u8, u32> = Default::default();
        let mut h: BTreeMap<u16, u32> = Default::default();
        let mut counter = 0;
        words.into_iter().for_each(|wx| {
            let w = wx.as_bytes();
            if w[0] == w[1] {
                *palindromes.entry(w[0]).or_default() += 1;
                return;
            }
            let w = [w[0] as u16, w[1] as u16];
            let rev = (w[0] << 8) + w[1];
            let mut flag = 0u8;
            h.entry(rev.clone()).and_modify(|r| {
                if *r == 1 {
                    flag = 2;
                } else {
                    *r -= 1;
                    flag = 1;
                }
                counter += 2;
            });
            match flag {
                2 => {
                    h.remove(&rev);
                }
                0 => {
                    // didn't find rev
                    let w = (w[1] << 8) + w[0];
                    *h.entry(w).or_default() += 1;
                }
                _ => {}
            }
        });
        let (mut pairs, mut single) = (0, false);
        for (_, c) in palindromes {
            if c % 2 == 0 {
                pairs += c >> 1;
            } else {
                if c > 1 {
                    pairs += c >> 1;
                }
                single = true;
            }
        }
        // println!("{} {} {}", counter, pairs, single);
        2 * (counter + 2 * pairs + if single { 1 } else { 0 }) as i32
    }
}
