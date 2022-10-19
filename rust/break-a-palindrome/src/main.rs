impl Solution {
    pub fn break_palindrome(mut palindrome: String) -> String {
        unsafe {
            let length = palindrome.len();
            let p = palindrome.as_bytes_mut();
            let mut mid = b'_';
            match length {
                x if x < 2 => {
                    return String::new();
                }
                x if x % 2 == 1 => {
                    mid = p[x / 2];
                    p[x / 2] = b'a';
                }
                _ => {}
            }
            let mut index = 114514;
            for i in 0..length / 2 {
                if p[i] != b'a' {
                    index = i;
                    break;
                }
            }
            if index == 114514 {
                p[p.len() - 1] = b'b';
            } else {
                p[index] = b'a';
            }
            if mid != b'_' {
                p[p.len() / 2] = mid;
            }
            palindrome
        }
    }
}

struct Solution {}
