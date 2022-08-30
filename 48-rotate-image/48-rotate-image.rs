impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        matrix.reverse();
        // 7 8 9
        // 4 5 6
        // 1 2 3
        let length = matrix.len();
        for i in 0..length {
            for j in (i+1)..length {
                let num = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = num;
            }
        }
    }
}
