const DIFF: u8 = b'a' - b'A';

impl Solution {
    pub fn make_good(mut s: String) -> String {
        unsafe {
            let bytes = s.as_bytes_mut();
            let (mut i, mut j, length) = (1, 1, bytes.len());
            let mut prev_char = bytes[0];
            while i < length {
                // println!("i is {}, bytes[i] is {}, j is {}, bytes[j] is {}, prev is {}", i, bytes[i] as char, j, bytes[j] as char, prev_char as char);
                if Solution::abs_diff(bytes[i], prev_char) == DIFF {
                    i += 1;
                    j -= 1;
                    if j > 0 {
                        prev_char = bytes[j - 1];
                    } else {
                        prev_char = 3;
                    }
                } else {
                    prev_char = bytes[i];
                    bytes[j] = prev_char;
                    i += 1;
                    j += 1;
                }
            }
            s.truncate(j);
            s
        }
    }
    fn abs_diff<T: std::ops::Sub<Output = T> + Ord>(x: T, y: T) -> T {
        if x < y {
            y - x
        } else {
            x - y
        }
    }
}