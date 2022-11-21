// 19:45 ~ 20:14

use std::collections::HashSet;

impl Solution {
    pub fn nearest_exit(mut maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let mut exits: HashSet<(u8, u8)> = HashSet::new();
        for (i, x) in maze[0].iter().enumerate() {
            if *x == '.' {
                exits.insert((0, i as u8));
            }
        }
        let last_i = maze.len() as u8 - 1;
        for (i, x) in maze[last_i as usize].iter().enumerate() {
            if *x == '.' {
                exits.insert((last_i, i as u8));
            }
        }
        let last_j = maze[0].len() - 1;
        let last_j_u8 = last_j as u8;
        for i in 0..maze.len() {
            if maze[i][0] == '.' {
                exits.insert((i as u8, 0));
            }
            if maze[i][last_j] == '.' {
                exits.insert((i as u8, last_j_u8));
            }
        }
        exits.remove(&(entrance[0] as u8, entrance[1] as u8));
        if exits.len() < 1 {
            return -1;
        }
        fn f(
            x: usize,
            y: usize,
            maze: &mut Vec<Vec<char>>,
            exits: &HashSet<(u8, u8)>,
            acc: usize,
        ) -> usize {
            if maze[x][y] != '.' {
                maze[x][y] = '=';
                return usize::MAX;
            }
            if exits.contains(&(x as u8, y as u8)) {
                return acc;
            }
            let acc = acc + 1;
            let mut res = usize::MAX;
            maze[x][y] = '&';
            if x + 1 < maze.len() {
                res = f(x + 1, y, maze, exits, acc).min(res);
            }
            if y + 1 < maze[0].len() {
                res = f(x, y + 1, maze, exits, acc).min(res);
            }
            if x > 0 {
                res = f(x - 1, y, maze, exits, acc).min(res);
            }
            if y > 0 {
                res = f(x, y - 1, maze, exits, acc).min(res);
            }
            maze[x][y] = '.';
            res
        }
        let ans = f(
            entrance[0] as usize,
            entrance[1] as usize,
            &mut maze,
            &exits,
            0,
        );
        if ans == usize::MAX {
            -1
        } else {
            ans as i32
        }
    }
}

struct Solution {}
fn main() {
    let res = Solution::nearest_exit(vec![vec!['.', '+']], vec![0, 0]);
    println!("{res}");
    let res = Solution::nearest_exit(
        vec![
            vec!['+', '+', '+'],
            vec!['.', '.', '.'],
            vec!['+', '+', '+'],
        ],
        vec![1, 0],
    );
    println!("{res}");
}
