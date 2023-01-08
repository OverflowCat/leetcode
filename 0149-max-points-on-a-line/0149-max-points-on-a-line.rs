use std::collections::HashMap;
impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n < 3 {
            return n as i32;
        }
        let mut res = 0;
        for (i, p_i) in points.iter().enumerate() {
            if res >= n - i || res > n / 2 {
                break;
            }
            let mut counter: HashMap<i32, u16> = Default::default();
            let (x_i, y_i) = (-p_i[0], -p_i[1]);
            for p_j in &points[i + 1..] {
                let (mut delta_x, mut delta_y) = (p_j[0] + x_i, p_j[1] + y_i);
                let negative = (delta_x < 0) ^ (delta_y < 0);
                delta_x = delta_x.abs();
                delta_y = delta_y.abs();
                let k = Self::gcd(delta_x, delta_y);
                delta_y /= k;
                delta_x /= k;
                if negative && delta_y != 0 {
                    delta_x = -delta_x;
                }
                // println!("{}, {}", delta_x, delta_y);
                *counter.entry(delta_x * 20001 + delta_y).or_default() += 1;
            }
            res = res.max(counter.into_values().max().unwrap_or_default() as usize);
        }
        res as i32 + 1
    }
    fn gcd(n1: i32, n2: i32) -> i32 {
        if n2 != 0 {
            Self::gcd(n2, n1 % n2)
        } else {
            n1
        }
    }
}
