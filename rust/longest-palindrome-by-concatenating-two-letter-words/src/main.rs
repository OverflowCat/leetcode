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
                // println!("*r is {}", *r);
                if *r == 1 {
                    flag = 2;
                } else {
                    *r -= 1;
                    flag = 1;
                }
                // println!("wx is {}", wx);
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

fn main() {
    assert_eq!(
        8,
        Solution::longest_palindrome(vec![
            "ab".into(),
            "ty".into(),
            "yt".into(),
            "lc".into(),
            "cl".into(),
            "ab".into()
        ])
    );
    assert_eq!(
        14,
        Solution::longest_palindrome(vec![
            "cc".into(),
            "cc".into(),
            "dd".into(),
            "aa".into(),
            "aa".into(),
            "dd".into(),
            "aa".into()
        ])
    );
    assert_eq!(
        10,
        Solution::longest_palindrome(vec![
            "cc".into(),
            "cc".into(),
            "dd".into(),
            "aa".into(),
            "aa".into(),
        ])
    );
    assert_eq!(
        10,
        Solution::longest_palindrome(vec![
            "aa".into(),
            "aa".into(),
            "aa".into(),
            "bb".into(),
            "bb".into(),
            "bb".into()
        ])
    );
    assert_eq!(
        26,
        Solution::longest_palindrome(vec![
            "ll".into(),
            "lb".into(),
            "bb".into(),
            "bx".into(),
            "xx".into(),
            "lx".into(),
            "xx".into(),
            "lx".into(),
            "ll".into(),
            "xb".into(),
            "bx".into(),
            "lb".into(),
            "bb".into(),
            "lb".into(),
            "bl".into(),
            "bb".into(),
            "bx".into(),
            "xl".into(),
            "lb".into(),
            "xx".into()
        ])
    );
}

struct Solution {}

/*
["cc","cc","dd","aa","aa"]
["cc","cc","dd","aa","aa","dd","aa"]
["cc","cc","dd","aa","aa","aa"]
["aa","aa","aa","bb","bb","bb"]
["aa","aa","aa","bb","bb","bb","cc","dd","dd","dd","dd","dd","dd","dd","dd","dd","dd","dd","dd","dd","dd"]
["ll","lb","bb","bx","xx","lx","xx","lx","ll","xb","bx","lb","bb","lb","bl","bb","bx","xl","lb","xx"]
lb lb lb lb bl 4 +2
bx xb bx bx 4    +2
lx lx xl 4       +2
ll ll 4
bb bb bb 6
xx xx xx 4
*/
ofvb