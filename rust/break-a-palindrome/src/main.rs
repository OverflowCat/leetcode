impl Solution {
    pub fn break_palindrome(mut palindrome: String) -> String {
        unsafe {
            let p = palindrome.as_bytes_mut();
            let mut mid = b'_';
            match p.len() {
                0 | 1 => {
                    return String::new();
                }
                x if x % 2 == 1 => {
                    mid = p[x / 2];
                    p[x / 2] = b'a';
                }
                _ => {}
            }
            let mut index: Option<usize> = None;
            for (i, &x) in p.iter().take(p.len() / 2).enumerate() {
                if x != b'a' {
                    index = Some(i);
                    break;
                }
            }
            if let Some(i) = index {
                p[i] = b'a';
            } else {
                p[p.len() - 1] = b'b';
            }
            if mid != b'_' {
                p[p.len() / 2] = mid;
            }
            palindrome
        }
    }
}

struct Solution {}
