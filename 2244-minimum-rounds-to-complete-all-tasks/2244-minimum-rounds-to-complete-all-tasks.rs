use std::collections::HashMap;

impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let mut hm: HashMap<i32, u32> = Default::default();
        for x in tasks {
            *hm.entry(x).or_default() += 1;
        }
        let mut res = 0;
        for (_, c) in hm {
            match Self::div_rem(c, 3) {
                (t, 0) => {res += t;}
                (0, 1) => {return -1;}
                (t, _) => {res += t + 1;}
            }
        }
        res as i32
    }

    fn div_rem<T>(x: T, y: T) -> (T, T) where T: std::ops::Div<Output=T> + std::ops::Rem<Output=T> + Copy {
        let quot = x / y;
        let rem = x % y;
        (quot, rem)
    }
}