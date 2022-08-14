impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        (1..n - 1)
            .map(|i| {
                (1..n - 1)
                    .map(|j| {
                        let mut largest = grid[i - 1][j - 1];
                        largest = largest.max(grid[i - 1][j]);
                        largest = largest.max(grid[i - 1][j + 1]);
                        largest = largest.max(grid[i][j - 1]);
                        largest = largest.max(grid[i][j]);
                        largest = largest.max(grid[i][j + 1]);
                        largest = largest.max(grid[i + 1][j - 1]);
                        largest = largest.max(grid[i + 1][j]);
                        largest = largest.max(grid[i + 1][j + 1]);
                        largest
                    })
                    .collect()
            })
            .collect()
    }
}
