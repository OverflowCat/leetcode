use std::{cmp::Ordering::*, collections::HashSet};
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Tree(i32, i32);
impl Solution {
    #[inline]
    fn cross_product(x_1: i32, y_1: i32, x_2: i32, y_2: i32) -> i32 {
        x_1 * y_2 - x_2 * y_1
    }

    pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if trees.len() < 3 { return trees; }
        let mut init = (114514, 114514);
        trees.iter().for_each(|t| match t[1].cmp(&init.1) {
            Less => { init = (t[0], t[1]); }
            Equal => { init.0 = init.0.min(t[0]); }
            _ => {}
        });
        let mut trees: Vec<Tree> = trees
            .into_iter()
            .map(|t| Tree(t[0] - init.0, t[1] - init.1))
            .collect();
        trees.sort_by(
            |a, b| match { 0.cmp(&Self::cross_product(a.0, a.1, b.0, b.1)) } {
                Equal => (a.0.pow(2) + a.1.pow(2)).cmp(&(b.0.pow(2) + b.1.pow(2))),
                res => res,
            },
        );
        let mut stack = Vec::new();
        stack.push(trees[0]);
        stack.push(trees[1]);
        for t in trees.iter().skip(2) {
            while {
                let length = stack.len();
                length > 1 && {
                    let (a, b) = (stack[stack.len() - 2], stack[stack.len() - 1]);
                    let x_1 = t.0 - a.0; let y_1 = t.1 - a.1;
                    let x_2 = b.0 - a.0; let y_2 = b.1 - a.1;
                    Self::cross_product(x_1, y_1, x_2, y_2) > 0
                }
            } { stack.pop().unwrap(); }
            stack.push(t.clone());
        }
        let mut trees: HashSet<_> = trees.into_iter().collect();
        for x in &stack {
            trees.remove(x);
        }
        let mut trees: Vec<_> = trees.into_iter().collect();
        let mut extra = vec![];
        stack.push(stack[0]);
        for x in stack.windows(2) {
            let Tree(p1x, p1y) = x[0];
            let Tree(p2x, p2y) = x[1];
            trees = trees.into_iter().filter_map(|a| {
                let Tree(p_x, p_y) = a;
                let vec1 = (p_x - p1x, p_y - p1y);
                let vec2 = (p_x - p2x, p_y - p2y);
                if (vec1.0 * vec2.0 + vec1.1 * vec2.1).pow(2)
                    == (vec1.0.pow(2) + vec1.1.pow(2)) * (vec2.0.pow(2) + vec2.1.pow(2))
                {
                    extra.push(a.clone());
                    None
                } else {
                    Some(a)
                }
            }).collect();
        }
        stack.pop();
        stack.into_iter().chain(extra.into_iter())
             .map(|x| vec![x.0 + init.0, x.1 + init.1]).collect()
    }
}