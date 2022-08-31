use std::collections::HashSet;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn dfs(
            heights: &Vec<Vec<i32>>,
            visited: &mut HashSet<(usize, usize)>,
            x: usize,
            y: usize,
            prev: i32,
        ) {
            if visited.contains(&(x, y)) {
                return;
            }
            let curr = heights[x][y];
            if curr < prev {
                return;
            }
            visited.insert((x, y));
            if x > 0 {
                dfs(heights, visited, x - 1, y, curr);
            }
            if y > 0 {
                dfs(heights, visited, x, y - 1, curr);
            }
            if x < heights.len() - 1 {
                dfs(heights, visited, x + 1, y, curr);
            }
            if y < heights[0].len() - 1 {
                dfs(heights, visited, x, y + 1, curr);
            }
        }
        let m = heights.len();
        let n = heights[0].len();
        let mut pacific = HashSet::new();
        for i in 0..m {
            dfs(&heights, &mut pacific, i, 0, 0);
        }
        for j in 1..n {
            dfs(&heights, &mut pacific, 0, j, 0);
        }
        let mut atlantic = HashSet::new();
        for i in 0..m {
            dfs(&heights, &mut atlantic, i, n - 1, 0);
        }
        for j in 0..n - 1 {
            dfs(&heights, &mut atlantic, m - 1, j, 0);
        }
        pacific
            .intersection(&atlantic)
            .map(|&(x, y)| vec![x as i32, y as i32])
            .collect()
    }
}
