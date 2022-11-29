use std::collections::HashMap;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut count: HashMap<i32, usize> = HashMap::new();
        let mut max = 0;
        matches.into_iter().for_each(|x| {
            *count.entry(x[1]).or_default() += 1;
            *count.entry(x[0]).or_insert(0);
        });
        let mut a = vec![];
        let mut b = vec![];
        for (k, v) in count.into_iter() {
            if v == 1 {
                b.push(k);
            } else if v == 0 {
                a.push(k);
            }
        }
        a.sort_unstable();
        b.sort_unstable();
        vec![a, b]
    }
}

struct Solution {}
