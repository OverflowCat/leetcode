use std::vec;

fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
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

fn main() {
    let data = vec![
        vec![9, 9, 8, 1],
        vec![5, 6, 2, 6],
        vec![8, 2, 6, 4],
        vec![6, 2, 2, 2],
    ];
    println!("{:?}", largest_local(data));

    let data = vec![
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 2, 1, 1],
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
    ];
    println!("{:?}", largest_local(data));
}
