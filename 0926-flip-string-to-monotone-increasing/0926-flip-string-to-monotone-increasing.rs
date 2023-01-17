impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let mut res = 0;
        let mut n = 0;
        s.into_bytes().into_iter().for_each(|b| {
            if b == b'0' {
                res = n.min(res + 1);
            } else {
                n += 1;
            }
        });
        res
    }
}
