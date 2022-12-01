impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let b_i = s.len() / 2;
        ({
            let mut count = 0;
            s.as_bytes().iter().take(b_i).for_each(|c| match c {
                b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U'
                => count += 1,
                _ => {}
            }); count
        }) == {
            let mut count = 0;
            s.as_bytes()[b_i..].into_iter().for_each(|c| match c {
                b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U'
                => count += 1,
                _ => {}
            }); count
        }
    }
}