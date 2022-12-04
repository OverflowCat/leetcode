impl Solution {
    fn frequency_sort(s: String) -> String {
        let mut char_counts = [0; 125];
        for &c in s.as_bytes() {
            char_counts[c as usize] += 1;
        }

        let mut chars: Vec<u8> = (0..125)
            .filter_map(|i| match char_counts[i] > 0 {
                true => Some(i as u8),
                _ => None,
            })
            .collect();
        chars.sort_by(|&a, &b| char_counts[b as usize].cmp(&char_counts[a as usize]));

        let mut result = String::new();
        for c in chars {
            result.push_str(&(c as char).to_string().repeat(char_counts[c as usize]));
        }
        result
    }
}

struct Solution {}

fn main() {
    let cases = vec![
        ("abcdefghijklmnopqrstuvwxyz", "zyxwvutsrqponmlkjihgfedcba"),
        ("aaaabbbccccddddeeeeffff", "ffffeeeeccccaaaaddddbbbb"),
        ("hello world", "loolhwdlro"),
        ("", ""),
        ("a", "a"),
        ("aa", "aa"),
        ("ab", "ab"),
        ("ba", "ba"),
    ];
    for (s, expected) in cases {
        assert_eq!(Solution::frequency_sort(s.to_string()), expected);
    }
}
