impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        use std::cmp::Ordering::Equal;
        use std::collections::HashMap;
        let mut counts: HashMap<String, usize> = HashMap::new();
        words.into_iter().for_each(|w| {
            *counts.entry(w).or_default() += 1;
        });
        let mut counts: Vec<(String, usize)> = counts.into_iter().collect();
        counts.sort_unstable_by(|a, b| match b.1.cmp(&a.1) {
            Equal => a.0.cmp(&b.0),
            x => x,
        });
        counts
            .into_iter()
            .take(k as usize)
            .map(|(word, _)| word)
            .collect()
    }
}
