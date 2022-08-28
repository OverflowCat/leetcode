fn print_matrix(mat: &Vec<Vec<i32>>) {
    for line in mat {
        for item in line {
            print!("{: >3} ", item);
        }
        println!();
    }
}

fn main() {
    let mat = vec![vec![3, 3, 1, 1], vec![2, 2, 1, 2], vec![1, 1, 1, 2]];
    print_matrix(&mat);
    let res = Solution::diagonal_sort(mat);
    print_matrix(&res);
    assert_eq!(
        res,
        vec![vec![1, 1, 1, 1], vec![1, 2, 2, 2], vec![1, 2, 3, 3]]
    );

    let mat = vec![
        vec![11, 25, 66, 1, 69, 7],
        vec![23, 55, 17, 45, 15, 52],
        vec![75, 31, 36, 44, 58, 8],
        vec![22, 27, 33, 25, 68, 4],
        vec![84, 28, 14, 11, 5, 50],
    ];
    print_matrix(&mat);
    let res = Solution::diagonal_sort(mat);
    print_matrix(&res);
    assert_eq!(
        res,
        vec![
            vec![5, 17, 4, 1, 52, 7],
            vec![11, 11, 25, 45, 8, 69],
            vec![14, 23, 25, 44, 58, 15],
            vec![22, 27, 31, 36, 50, 66],
            vec![84, 28, 75, 33, 55, 68]
        ]
    );

    let mat = vec![vec![99]];
    let res = Solution::diagonal_sort(mat);
    assert_eq!(res, vec![vec![99]]);
}

struct Solution {}

impl Solution {
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
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
