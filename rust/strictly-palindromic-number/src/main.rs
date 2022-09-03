use std::collections::VecDeque;

impl Solution {
    pub fn is_strictly_palindromic(n: i32) -> bool {
        (2..n - 1).all(|base| {
            let mut res: Vec<i32> = Vec::new();
            let mut n = n;
            while n != 0 {
                res.push(n % base);
                n /= base;
            }
            res.iter()
                .rev()
                .zip(res.iter())
                .take(res.len() / 2 + 1)
                .all(|(&f, &b)| f == b)
        })
    }
}
