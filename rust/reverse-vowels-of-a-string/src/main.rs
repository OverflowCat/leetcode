impl Solution {
    pub fn reverse_vowels(mut s: String) -> String {
        let (mut i, mut j) = (0, s.len() - 1);
        unsafe {
            let b = s.as_bytes_mut();
            while i < j {
                while i < j {
                    match b[i] {
                        b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U' => {
                            break;
                        }
                        _ => {
                            i += 1;
                        }
                    }
                }
                while i < j {
                    match b[j] {
                        b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U' => {
                            break;
                        }
                        _ => {
                            j -= 1;
                        }
                    }
                }
                if i >= j {
                    break;
                }
                if i < j {
                    b.swap(i, j);
                    i += 1;
                    j -= 1;
                }
            }
            s
        }
    }
}

struct Solution {}

fn main() {
    assert_eq!(
        String::from("leotcede"),
        Solution::reverse_vowels("leetcode".into())
    );
}
