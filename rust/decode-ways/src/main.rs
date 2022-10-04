impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let length = s.len();
        let s = s.as_bytes();
        if s[0] == b'0' {
            return 0;
        }
        if length == 1 {
            return 1;
        }
        let mut ways = vec![0; length + 1];
        ways[0] = 1;
        ways[1] = 1;
        for i in 1..length {
            if s[i] == 0 {
                if s[i - 1] != b'1' && s[i - 1] != b'2' {
                    return 0;
                }
            } else {
                ways[i + 1] = ways[i];
            }
            if s[i - 1] == b'1' || s[i - 1] == b'2' && s[i] < b'7' {
                ways[i + 1] += ways[i - 1];
            }
        }
        ways[length]
    }
}
