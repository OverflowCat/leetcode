impl Solution {
    pub fn daily_temperatures(mut temperatures: Vec<i32>) -> Vec<i32> {
        let (mut stack, n) = (vec![], temperatures.len());
        for i in 0..n {
            // println!("i: {:?}", stack);
            while let Some(&j) = stack.last() {
                if temperatures[j] < temperatures[i] {
                    // dbg!(j, temperatures[j]);
                    temperatures[j] = (i - stack.pop().unwrap()) as i32;
                } else {
                    break;
                }
            }
            stack.push(i);
        }
        stack.into_iter().for_each(|x| {
            temperatures[x] = 0;
        });
        temperatures
    }
}