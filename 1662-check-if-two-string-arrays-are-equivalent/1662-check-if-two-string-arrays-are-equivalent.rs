impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1.iter().map(|s| s.len()).sum::<usize>() == word2.iter().map(|s| s.len()).sum::<usize>()
            && word1
                .join("")
                .bytes()
                .zip(word2.join("").bytes())
                .all(|(a, b)| a == b)
    }
}