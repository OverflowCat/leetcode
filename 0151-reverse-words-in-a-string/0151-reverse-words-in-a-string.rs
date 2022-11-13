impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_ascii_whitespace().rev().into_iter().collect::<Vec<_>>().join(" ")
    }
}