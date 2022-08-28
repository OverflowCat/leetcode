impl Solution {
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len(); // 行数
        let n = mat[0].len(); // 列数
        let mut mat = mat;
        for x in 0..n {
            let (mut i, mut j) = (m - 1, x);
            let mut this_line: Vec<i32> = vec![];
            loop {
                this_line.push(mat[i][j]);
                if i == 0 || j == 0 {
                    break;
                }
                i -= 1;
                j -= 1;
            }
            this_line.sort();
            // this_line.reverse();

            let (mut i, mut j) = (m - 1, x);
            loop {
                mat[i][j] = this_line.pop().unwrap();
                if i == 0 || j == 0 {
                    break;
                }
                i -= 1;
                j -= 1;
            }
        }

        // println!("m {}, n {}", m, n);
        for y in (0..m - 1).rev() {
            let (mut i, mut j) = (y, n - 1);
            let mut this_line: Vec<i32> = vec![];
            loop {
                this_line.push(mat[i][j]);
                if i == 0 || j == 0 {
                    break;
                }
                i -= 1;
                j -= 1;
            }
            this_line.sort();
            // println!("This line is {:?}", this_line);
            let (mut i, mut j) = (y, n - 1);
            loop {
                mat[i][j] = this_line.pop().unwrap();
                if i == 0 || j == 0 {
                    break;
                }
                i -= 1;
                j -= 1;
            }
        }
        mat
    }
}
