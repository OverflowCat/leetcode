impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        matrix.windows(2).into_iter().all(|lines| {
            lines[0]
                .iter()
                .zip(lines[1].iter().skip(1))
                .all(|(a, b)| a == b)
        })
    }
}