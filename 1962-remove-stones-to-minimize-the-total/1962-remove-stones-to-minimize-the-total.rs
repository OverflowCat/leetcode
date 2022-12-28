impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, mut k: i32) -> i32 {
        let mut ans = 0;
        let mut pil = [0; 10001];
        piles.iter().for_each(|&x| {
            pil[x as usize] += 1;
            ans += x;
        });
        let mut i = 10000;
        while k > 0 {
            if pil[i] == 0 {
                i -= 1;
            } else {
                ans -= (i / 2) as i32;
                pil[i] -= 1;
                pil[(i + 1) / 2] += 1;
                k -= 1;
            }
        }
        ans
    }
}