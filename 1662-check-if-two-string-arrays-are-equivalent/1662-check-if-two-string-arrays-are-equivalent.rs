impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let a = word1.join("");
        let b = word2.join("");
        a.len() == b.len() && a.bytes().zip(b.bytes()).all(|(a, b)| a == b)
    }
}