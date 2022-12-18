impl Solution {
    pub fn daily_temperatures(mut temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        for i in 0..temperatures.len() {
            while let Some(&j) = stack.last() {
                if temperatures[j] < temperatures[i] {
                    temperatures[j] = (i - j) as i32;
                    temperatures.pop();
                } else {
                    break;
                }
            }
            stack.push(i);
        }
        temperatures
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
