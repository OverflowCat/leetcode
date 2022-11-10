impl Solution {
    pub fn remove_duplicates(mut s: String) -> String {
        unsafe {
            let bytes = s.as_bytes_mut();
            let (mut i, mut j, length) = (1, 1, bytes.len());
            let mut prev_char = bytes[0];
            while i < length {
                if bytes[i] == prev_char {
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
}