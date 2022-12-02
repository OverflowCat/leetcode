impl Solution {
    fn letters(word: String) -> (usize, [usize; 26]) {
        let mut counts = [0usize; 26];
        word.as_bytes().into_iter().for_each(|b| {
            counts[(b - b'a') as usize] += 1;
        });
        let mut chars = 0usize;
        counts.iter().enumerate().for_each(|(i, &c)| match c {
            _ if c == 0 => {}
            _ => {
                chars += 1 << i;
            }
        });
        (chars, counts)
    }
    pub fn close_strings(word1: String, word2: String) -> bool {
        word1.len() == word2.len() && {
            let (chars1, mut counts1) = Solution::letters(word1);
            let (chars2, mut counts2) = Solution::letters(word2);
            chars1 == chars2 && {
                counts1.sort_unstable();
                counts2.sort_unstable();
                counts1 == counts2
            }
        }
    }
}
