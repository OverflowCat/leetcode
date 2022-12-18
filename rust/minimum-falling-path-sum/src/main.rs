impl Solution {
    pub fn min_falling_path_sum(mut matrix: Vec<Vec<i32>>) -> i32 {
        let width = matrix[0].len() - 1;
        for i in 1..matrix.len() {
            matrix[i][0] += matrix[i - 1][0].min(matrix[i - 1][1]);
            for j in 1..width {
                matrix[i][j] += matrix[i - 1][j - 1]
                    .min(matrix[i - 1][j])
                    .min(matrix[i - 1][j + 1]);
            }
            matrix[i][width] += matrix[i - 1][width - 1].min(matrix[i - 1][width]);
        }
        *matrix.last().unwrap().into_iter().min().unwrap()
    }
}
fn main() {
    println!("Hello, world!");
}
