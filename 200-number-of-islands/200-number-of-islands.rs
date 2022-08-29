impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mmax = grid.len() - 1;
        let nmax = grid[0].len() - 1;
        let mut counter = 0;
        let mut grid = grid;
        for i in 0..=mmax {
            for j in 0..=nmax {
                if grid[i][j] == '1' {
                    counter += 1;
                    dfs(i, j, &mut grid, mmax, nmax);
                }
            }
        }

        fn dfs(i: usize, j: usize, grid: &mut Vec<Vec<char>>, mmax: usize, nmax: usize) {
            if grid[i][j] == '1' {
                grid[i][j] = '2';
                if i > 0 {
                    dfs(i - 1, j, grid, mmax, nmax);
                }
                if j > 0 {
                    dfs(i, j - 1, grid, mmax, nmax);
                }
                if i < mmax {
                    dfs(i + 1, j, grid, mmax, nmax);
                }
                if j < nmax {
                    dfs(i, j + 1, grid, mmax, nmax);
                }
            }
        }

        counter
    }
}
