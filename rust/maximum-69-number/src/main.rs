use std::str::from_utf8_unchecked;

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        unsafe {
            let n = num.to_string().as_bytes_mut();
            for digit in n {
                if *digit == b'6' {
                    *digit = b'9';
                }
            }
            str::parse::<i32>(num).unwrap()
        }
    }
}
