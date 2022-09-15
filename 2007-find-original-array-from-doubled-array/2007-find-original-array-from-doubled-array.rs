use std::{collections::HashMap, vec};
impl Solution {
    pub fn find_original_array(mut changed: Vec<i32>) -> Vec<i32> {
        if changed.len() % 2 != 0 {
            return vec![];
        }
        changed.sort_unstable();
        let mut unused: HashMap<i32, usize> = HashMap::new();
        changed
            .iter()
            .for_each(|&x| *unused.entry(x).or_insert(0) += 1);
        let mut res: Vec<i32> = Vec::with_capacity(changed.len() / 2);
        for x in changed {
            if !unused.contains_key(&x) {
                continue;
            }
            let doubled = &(2 * x);
            match unused.get(doubled) {
                Some(1) => {
                    unused.remove(doubled);
                }
                Some(_) => {
                    unused.entry(*doubled).and_modify(|v| *v -= 1);
                }
                None => {
                    return vec![];
                }
            }
            match unused.get(&x) {
                Some(1) => {
                    unused.remove(&x);
                }
                Some(_) => {
                    unused.entry(x).and_modify(|v| *v -= 1);
                }
                None => {
                    return vec![];
                }
            }
            res.push(x);
        }
        if unused.len() == 0 {
            res
        } else {
            vec![]
        }
    }
}
