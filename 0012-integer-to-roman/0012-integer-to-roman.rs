impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let num = num as usize;
        fn f(val: usize, single: &str, half: &str, lar: &str) -> String {
            match val {
                4 => single.to_owned() + half,
                9 => single.to_owned() + lar,
                n if n < 4 => single.repeat(n),
                n => half.to_owned() + &single.repeat(n - 5),
            }
        }
        format!(
            "{}{}{}{}",
            "M".repeat(num / 1000),
            f((num / 100) % 10, "C", "D", "M"),
            f((num / 10) % 10, "X", "L", "C"),
            f(num % 10, "I", "V", "X")
        )
    }
}
