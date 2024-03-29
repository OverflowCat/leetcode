impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut left = 0usize;
        for c in data {
            match left {
                0 => match c {
                    _ if c < 0b1_0000000 => {}
                    _ if c < 0b11_000000 => {
                        return false;
                    }
                    _ if c < 0b111_00000 => {
                        left = 1;
                    }
                    _ if c < 0b1111_0000 => {
                        left = 2;
                    }
                    _ if c < 0b11111_000 => {
                        left = 3;
                    }
                    _ => {
                        return false;
                    }
                },
                _ => {
                    if c >= 0b1_0000000 && c < 0b11_000000 {
                        if left == 0 {
                            return false;
                        }
                        left -= 1;
                    } else {
                        return false;
                    }
                }
            }
        }
        left == 0
    }
}