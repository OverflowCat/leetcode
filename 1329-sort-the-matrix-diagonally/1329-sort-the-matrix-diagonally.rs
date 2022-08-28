impl Solution {
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (mat.len(), mat[0].len());
        let mut mat = mat;
        for x in 0..n {
            let mut this_line = vec![];
            for i in 0..=x.min(m - 1) {
                this_line.push(mat[m - 1 - i][x - i]);
            }
            this_line.sort_unstable();
            for i in 0..=x.min(m - 1) {
                mat[m - 1 - i][x - i] = this_line.pop().unwrap();
            }
        }

        for y in (0..m - 1).rev() {
            let mut this_line: Vec<i32> = vec![];
            for i in 0..=y.min(n - 1) {
                this_line.push(mat[y - i][n - 1 - i]);
            }
            this_line.sort_unstable();
            for i in 0..=y.min(n - 1) {
                mat[y - i][n - 1 - i] = this_line.pop().unwrap();
            }
        }
        mat
    }
}
