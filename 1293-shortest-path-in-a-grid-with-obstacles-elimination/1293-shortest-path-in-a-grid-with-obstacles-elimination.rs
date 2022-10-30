use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let (l1, l2, k) = (grid.len(), grid[0].len(), k as usize);
        if (l1, l2) == (1, 1) {
            return 0;
        }
        if k >= l1 + l2 - 3 {
            return (l1 + l2 - 2) as i32;
        }
        let k = k as u16;
        let mut q: VecDeque<_> = VecDeque::new();
        q.push_back((0, 0, 0, 0));
        let mut visited: HashSet<(usize, usize, u16)> = HashSet::with_capacity(l1 * l2);
        visited.insert((0, 0, 0));
        while !q.is_empty() {
            let (x, y, n, ob) = q.pop_front().unwrap();
            if x == l1 - 1 && y == l2 - 1 {
                return n as i32;
            }
            macro_rules! f {
                ($nx:expr, $ny:expr) => {
                    let nob = ob + grid[$nx][$ny] as u16;
                    if nob <= k && visited.insert(($nx, $ny, nob)) {
                        q.push_back(($nx, $ny, n + 1, nob));
                    }
                };
            }
            if x + 1 < l1 {
                f!(x + 1, y);
            }
            if y + 1 < l2 {
                f!(x, y + 1);
            }
            if x > 0 {
                f!(x - 1, y);
            }
            if y > 0 {
                f!(x, y - 1);
            }
        }
        -1
    }
}
