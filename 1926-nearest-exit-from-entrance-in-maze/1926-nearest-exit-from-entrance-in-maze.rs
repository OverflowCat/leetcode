// 19:45 ~ 20:30

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn nearest_exit(mut maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let mut exits: HashSet<(usize, usize)> = HashSet::new();
        for (i, x) in maze[0].iter().enumerate() {
            if *x == '.' {
                exits.insert((0, i));
            }
        }
        let last_i = maze.len() - 1;
        for (i, x) in maze[last_i as usize].iter().enumerate() {
            if *x == '.' {
                exits.insert((last_i, i));
            }
        }
        let last_j = maze[0].len() - 1;
        for i in 0..maze.len() {
            if maze[i][0] == '.' {
                exits.insert((i, 0));
            }
            if maze[i][last_j] == '.' {
                exits.insert((i, last_j));
            }
        }
        exits.remove(&(entrance[0] as usize, entrance[1] as usize));
        if exits.len() < 1 {
            return -1;
        }
        let mut min = usize::MAX;
        let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
        queue.push_back((entrance[0] as usize, entrance[1] as usize, 0));
        while !queue.is_empty() {
            let (x, y, acc) = queue.pop_front().unwrap();
            if maze[x][y] != '.' {
                maze[x][y] = '=';
                continue;
            }
            if exits.contains(&(x, y)) {
                min = acc.min(min);
            }
            let acc = acc + 1;
            // if acc > min {
            //     continue;
            // };
            maze[x][y] = '&';
            if x + 1 < maze.len() {
                queue.push_back((x + 1, y, acc));
            }
            if y + 1 < maze[0].len() {
                queue.push_back((x, y + 1, acc));
            }
            if x > 0 {
                queue.push_back((x - 1, y, acc));
                queue.push_back((x - 1, y, acc));
            }
            if y > 0 {
                queue.push_back((x, y - 1, acc));
            }
        }
        if min == usize::MAX {
            -1
        } else {
            min as i32
        }
    }
}