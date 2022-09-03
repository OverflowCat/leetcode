fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        if k == 0 {
            (1..10)
                .map(|d| {
                    let mut r = 0;
                    for _ in 0..n {
                        r = r * 10 + d;
                    }
                    r
                })
                .collect()
        } else {
            fn f(gen: i32, last: i32, left: i32, res: &mut Vec<i32>, k: &i32) {
                if left == 0 {
                    res.push(gen);
                    return;
                }
                let next = last + k;
                if next < 10 {
                    f(gen * 10 + next, next, left - 1, res, k);
                }
                let next = last - k;
                if next > -1 {
                    f(gen * 10 + next, next, left - 1, res, k);
                }
            }
            let mut res: Vec<i32> = vec![];
            for i in 1..10 {
                f(i, i, n - 1, &mut res, &k);
            }
            res
        }
    }
}
