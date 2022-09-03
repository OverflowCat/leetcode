use std::collections::HashSet;
impl Solution {
    pub fn maximum_rows(mat: Vec<Vec<i32>>, cols: i32) -> i32 {
        let (rows_count, cols_count) = (mat.len(), mat[0].len());
        let rows = (0..cols_count)
            .map(|c| (0..rows_count).map(|r| mat[r][c]).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut sum_of_rows: Vec<(usize, i32)> = rows
            .iter()
            .enumerate()
            .map(|(i, v)| (i, v.iter().sum()))
            .collect();
        sum_of_rows.sort_unstable_by_key(|x| x.1);
        sum_of_rows.reverse();
        // println!("{:?}", sum_of_rows); 
        let del: HashSet<usize> = sum_of_rows.into_iter().take(cols as usize).map(|x| x.0).collect();
        // println!("{:?}", del); 
        mat.iter()
            .filter(|row| {
                row.iter()
                    .enumerate()
                    .all(|(i, &val)| val == 0 || del.contains(&i))
            })
            .count() as i32
    }
}
