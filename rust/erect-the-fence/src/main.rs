use std::cmp::Ordering::*;
#[derive(Copy, Clone)]
struct Tree(i32, i32, f64);
impl Solution {
    #[inline]
    fn cross_product(x_1: i32, y_1: i32, x_2: i32, y_2: i32) -> i32 {
        x_1 * y_2 - x_2 * y_1
    }

    pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 找到一个必定属于凸包的点，一般是纵坐标最小的点，若有多个再取横坐标最小，记为P0。
        let mut init = (114514, 114514);
        trees.iter().for_each(|t| match t[1].cmp(&init.1) {
            Less => {
                init = (t[0], t[1]);
            }
            Equal => {
                init.0 = init.0.min(t[0]);
            }
            _ => {}
        });
        let mut trees: Vec<Tree> = trees
            .into_iter()
            .map(|t| {
                let (x, y) = (t[0] - init.0, t[1] - init.1);
                Tree(
                    x,
                    y,
                    if x == 0 {
                        f64::MAX
                    } else {
                        y as f64 / x as f64
                    },
                )
            })
            .collect();
        trees.sort_by(|a, b| match a.2.partial_cmp(&b.2).unwrap() {
            Equal => (a.0.pow(2) + a.1.pow(2)).cmp(&(b.0.pow(2) + b.1.pow(2))),
            res => res,
        });
        let mut stack = Vec::new();
        stack.push(trees[0]);
        stack.push(trees[1]);
        for t in trees.into_iter().skip(2) {
            let a = stack[stack.len() - 2];
            let b = stack[stack.len() - 1];
            let x_1 = t.0 - a.0;
            let y_1 = t.1 - a.1;
            let x_2 = b.0 - a.0;
            let y_2 = b.1 - a.1;
            if Self::cross_product(x_1, y_1, x_2, y_2) < 0 {
                stack.pop();
            }
            stack.push(t);
            /*
            ①若结果大于0，则AC在AB的逆时针方向
            ②若结果小于0，则AC在AB的顺时针方向
            ③若结果等于0，则AC与AB平行（其实是共线，但是可能朝向相反） */
        }
        stack
            .iter()
            .map(|x| vec![x.0 + init.0, x.1 + init.1])
            .collect()
    }
}
struct Solution;

fn main() {
    use std::collections::HashSet;
    let res: HashSet<Vec<i32>> = HashSet::from_iter(
        Solution::outer_trees(vec![
            vec![1, 1],
            vec![2, 2],
            vec![2, 0],
            vec![2, 4],
            vec![3, 3],
            vec![4, 2],
        ])
        .into_iter(),
    );
    assert_eq!(
        res,
        HashSet::from_iter(
            vec![vec![1, 1], vec![2, 0], vec![4, 2], vec![3, 3], vec![2, 4]].into_iter()
        )
    );
}
