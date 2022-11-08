use std::str::from_utf8_unchecked;

impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        let mut s = s.into_bytes();
        if k == 1 {
            let mut r = s.clone();
            for _ in 1..s.len() {
                r.rotate_left(1); // 这里是不是可以优化，不移动字符串？
                println!("{:?}", r);
                if r < s {
                    s = r.clone();
                }
            }
        } else {
            s.sort_unstable();
        }
        unsafe { from_utf8_unchecked(&s).to_owned() }
    }
}

fn main() {}
struct Solution {}
