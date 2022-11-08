const DIFF: u8 = b'a' - b'A';

impl Solution {
    pub fn make_good(mut s: String) -> String {
        unsafe {
            let bytes = s.as_bytes_mut();
            let mut j = 1;
            let mut prev_i = bytes[0];
            for i in 1..bytes.len() {
                if bytes[i].abs_diff(prev_i) == DIFF {
                    j -= 1;
                } else {
                    bytes[j] = bytes[i];
                    prev_i = bytes[i];
                    j += 1;
                }
            }
            println!("s is {}", s);
            println!("j is {}", j);
            s.get_unchecked_mut(0..j).to_owned()
        }
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::make_good("leEeetcode".into()), "leetcode");
    // l*eE*etcode leetcode
    assert_eq!(Solution::make_good("abBAcC".into()), "");
}
