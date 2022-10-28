use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<u32, Vec<String>> = HashMap::new();
        strs.into_iter().for_each(|s| {
            let mut used = 0u32;
            s.as_bytes().into_iter().for_each(|b| {
                used |= 1 << b - b'a';
            });
            (*map.entry(used).or_insert(vec![])).push(s);
        });
        map.into_values().collect()
    }
}
