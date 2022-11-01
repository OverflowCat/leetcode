impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (grid.len(), grid[0].len());
        let mut res = Vec::with_capacity(n);
        for i in 0..n {
            let mut f: Option<i32> = None;
            let mut x = i;
            for j in 0..m {
                if grid[j][x] == 1 {
                    // 1　＼
                    if x == n - 1 || grid[j][x + 1] == -1 {
                        f = Some(-1);
                        break;
                    }
                    x += 1;
                } else {
                    // -1　／
                    if x == 0 || grid[j][x - 1] == 1 {
                        f = Some(-1);
                        break;
                    }
                    x -= 1;
                }
            }
            res.push(f.unwrap_or(x as i32));
        }
        res
    }
}
