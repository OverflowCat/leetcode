impl Solution {
    pub fn min_cost(mut colors: String, needed_time: Vec<i32>) -> i32 {
        let (mut time, mut count, mut prev) = (0, 0, 0);
        colors.push('_');
        colors.as_bytes().iter().enumerate().for_each(|(i, &c)| {
            if c == prev {
                count += 1;
            } else if count != 0 {
                let slice = &needed_time[i - count - 1..i];
                let sum: i32 = slice.iter().sum();
                time += sum - slice.iter().max().unwrap();
                count = 0;
            }
            prev = c;
        });
        time
    }
}
