impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn f(i: usize, graph: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            if i == graph.len() - 1 {
                vec![vec![i as i32]]
            } else {
                let mut res = vec![];
                for c in &graph[i] {
                    for mut p in f(*c as usize, graph) {
                        let mut new = Vec::with_capacity(p.len() + 1);
                        new.push(i as i32);
                        new.append(&mut p);
                        res.push(new);
                    }
                }
                res
            }
        }
        f(0, &graph)
    }
}