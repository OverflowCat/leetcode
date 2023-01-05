impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        let (mut i, mut count, n) = (0, 0, points.len());
        points.sort_unstable_by_key(|x| x[0]);
        while i < n {
            let (mut l, mut r) = (points[i][0], points[i][1]);
            let mut j = i + 1;
            while j < n && points[j][0] <= r { // 注意有等于
                l = l.max(points[j][0]);
                r = r.min(points[j][1]);
                j += 1;
            }
            count += 1;
            i = j;
        }
        count as i32
    }
}