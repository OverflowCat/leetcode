impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut count = [0usize; 10001];
        let mut max = 0usize;
        for &num in nums.iter() {
            let num = num as usize;
            count[num] += num;
            max = max.max(num);
        }
        for i in 2..=max {
            count[i] = count[i - 1].max(count[i - 2] + count[i]);
        }
        count[max] as i32
    }
}
