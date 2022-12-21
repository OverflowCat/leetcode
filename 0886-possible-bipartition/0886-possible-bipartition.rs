impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        dislikes.into_iter().for_each(|it| {
            let (x, y) = (it[0] as usize - 1, it[1] as usize - 1);
            graph[x].push(y);
            graph[y].push(x);
        });
        let mut v = vec![0; n];
        fn f(x: usize, c: i32, v: &mut Vec<i32>, g: &mut Vec<Vec<usize>>) -> bool {
            v[x] = c;
            (0..g[x].len()).all(|j| {
                let y = g[x][j];
                v[y] != c && (v[y] != 0 || f(y, -c, v, g))
            })
        }
        for i in 0..n {
            let x = v[i];
            if x == 0 && !f(i, 1, &mut v, &mut graph) {
                return false;
            }
        }
        true
    }
}