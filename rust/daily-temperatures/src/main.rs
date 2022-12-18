impl Solution {
    pub fn daily_temperatures(mut temperatures: Vec<i32>) -> Vec<i32> {
        let (mut stack, n) = (vec![], temperatures.len());
        for i in 0..n {
            while let Some(&j) = stack.last() {
                if temperatures[j] < temperatures[i] {
                    temperatures[j] = (i - stack.pop().unwrap()) as i32;
                } else {
                    break;
                }
            }
            stack.push(i);
        }
        &mut temperatures[n - stack.len() - 1..].fill(0);
        temperatures
    }
}
