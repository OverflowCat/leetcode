impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n < 4 {
            n
        } else {
            let (mut a, mut b) = (1, 2);
            for x in 3..=n {
                if a > b {
                    b += a;
                } else {
                    a += b;
                }
            }
            a.max(b)
        }
    }
}