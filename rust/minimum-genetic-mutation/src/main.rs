use std::collections::VecDeque;
impl Solution {
    pub fn min_mutation(start: String, end: String, mut bank: Vec<String>) -> i32 {
        if !bank.contains(&end) { return -1; }
        let mut q = VecDeque::with_capacity(bank.len());
        bank.push(end.clone());
        q.push_back((start, 0));
        loop {
            if let Some((s, t)) = q.pop_front() {
                println!("{} {}", s, t);
                if s == end { return t; }
                let t = t + 1;
                { //drain filter
                    let mut i = 0;
                    while i < bank.len() {
                        if Solution::diff(&bank[i], &s) {
                            q.push_back((bank.remove(i), t));
                        } else {
                            i += 1;
                        }
                    }
                }
            } else { return -1; }
        }
    }

    #[inline(always)]
    pub fn diff(a: &String, b: &String) -> bool {
        let mut flag = false;
        for (&x, &y) in a.as_bytes().iter().zip(b.as_bytes().iter()) {
            if x != y {
                if flag { return false; }
                flag = true;
            }
        }
        flag
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::diff(&"AACCGGTT".into(), &"AACCGGTA".into()), true);
    let start = "AACCGGTT".into();
    let end = "AACCGGTA".into();
    let bank = vec!["AACCGGTA".into()];
    Solution::min_mutation(start, end, bank);
    let start = "TTTTTATG".into();
    let end = "TTTTTTTG".into();
    let bank = vec!["CCCTTCCC".into()];
    assert_eq!(Solution::min_mutation(start, end, bank), -1);
}
