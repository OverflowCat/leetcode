impl Solution {
    pub fn hammingWeight (n: u32) -> i32 {
        let mut n = n;
        let mut c: u32 = 0;
        while n != 0 {
            c += n % 2;
            n /= 2;
        }
        return c as i32;
    }
}
