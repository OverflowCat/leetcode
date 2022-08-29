impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let n = grid[0].len();
        let mut counter = 0;
        for i in 0..grid.len() {
            for j in 0..n {
                if grid[i][j] == '1' {
                    counter += 1;
                    dfs(i, j, &mut grid);
                }
            }
        }

        fn dfs(i: usize, j: usize, grid: &mut Vec<Vec<char>>) {
            if grid[i][j] == '1' {
                grid[i][j] = '2';
                if i > 0 {
                    dfs(i - 1, j, grid);
                }
                if j > 0 {
                    dfs(i, j - 1, grid);
                }
                if i < grid.len() - 1 {
                    dfs(i + 1, j, grid);
                }
                if j < grid[0].len() - 1 {
                    dfs(i, j + 1, grid);
                }
            }
        }

        counter
    }
}
