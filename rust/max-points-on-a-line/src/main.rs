use std::collections::HashMap;
impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        pub fn gcd(mut u: u16, mut v: u16) -> u16 {
            // from Wikipedia
            use std::cmp::min;
            use std::mem::swap;
            if u == 0 {
                return v;
            } else if v == 0 {
                return u;
            }
            let i = u.trailing_zeros();
            u >>= i;
            let j = v.trailing_zeros();
            v >>= j;
            let k = min(i, j);
            loop {
                if u > v {
                    swap(&mut u, &mut v);
                }
                v -= u;
                if v == 0 {
                    return u << k;
                }
                v >>= v.trailing_zeros();
            }
        }
        let n = points.len();
        if n < 3 {
            return n as i32;
        }
        let mut res = 0;
        for (i, p_i) in points.iter().enumerate() {
            let mut counter: HashMap<i32, u16> = Default::default();
            let (x_i, y_i) = (-p_i[0], -p_i[1]);
            for p_j in &points[i + 1..] {
                let (mut delta_y, mut delta_x) = (p_j[0] + x_i, p_j[1] + y_i);
                let negative = (delta_x < 0) ^ (delta_y < 0);
                let mut k = gcd(delta_y.abs() as u16, delta_x.abs() as u16) as i32;
                if negative { k = -k; }
                delta_y /= k;
                delta_x /= k;
                *counter.entry(delta_x * 20001 + delta_y).or_default() += 1;
            }
            res = res.max(counter.into_values().max().unwrap_or_default());
        }
        res as i32
    }
}
